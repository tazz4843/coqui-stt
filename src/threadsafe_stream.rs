use crate::{Metadata, Model, Result, Stream};
use flume::{Receiver, Sender};
use std::sync::Arc;

/// A thread-safe wrapper around a [`Stream`](crate::Stream).
pub struct ThreadSafeStream {
    sender: Sender<Box<dyn FnOnce(&mut Stream) + Send + Sync>>,
    owned_sender: Sender<Box<dyn FnOnce(Stream) + Send + Sync>>,
}

impl ThreadSafeStream {
    /// Create a new threadsafe stream.
    ///
    /// # Errors
    /// Returns any errors [`Stream::from_model`](crate::Stream::from_model) returns.
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn new(model: Arc<Model>) -> Result<Self> {
        let (init_tx, init_rx) = flume::bounded(1);
        let (unowned_tx, unowned_rx) =
            flume::bounded::<Box<dyn FnOnce(&mut Stream) + Send + Sync>>(1024);
        let (owned_tx, owned_rx) = flume::bounded::<Box<dyn FnOnce(Stream) + Send + Sync>>(1);

        std::thread::spawn(move || {
            let mut stream = match Stream::from_model(model) {
                Ok(s) => {
                    let _send_res = init_tx.send(Ok(()));
                    s
                }
                Err(e) => {
                    let _send_res = init_tx.send(Err(e));
                    return;
                }
            };
            drop(init_tx);

            while let Ok(f) = unowned_rx.recv() {
                f(&mut stream);

                if let Ok(f) = owned_rx.try_recv() {
                    f(stream);
                    return;
                }
            }
        });
        init_rx
            .recv()
            .expect("no senders should be dropped unless spawning thread failed")?;

        Ok(Self {
            sender: unowned_tx,
            owned_sender: owned_tx,
        })
    }

    fn send_and_get_recv<T: 'static + Send>(
        &self,
        f: Box<dyn FnOnce(&mut Stream) -> T + Send + Sync>,
    ) -> Receiver<T> {
        let (tx, rx) = flume::bounded(1);
        let f = move |stream: &mut Stream| {
            let _send_res = tx.send(f(stream));
        };

        self.sender
            .send(Box::new(f))
            .expect("background thread panicked");
        rx
    }

    fn send_and_get_recv_move<T: 'static + Send>(
        self,
        f: Box<dyn FnOnce(Stream) -> T + Send + Sync>,
    ) -> Receiver<T> {
        let (tx, rx) = flume::bounded(1);
        let f = move |stream: Stream| {
            let _send_res = tx.send(f(stream));
        };

        self.sender
            .send(Box::new(|_stream| {}))
            .expect("background thread panicked");
        self.owned_sender
            .send(Box::new(f))
            .expect("background thread panicked");
        rx
    }

    #[inline]
    fn send_and_get<T: 'static + Send>(
        &self,
        f: Box<dyn FnOnce(&mut Stream) -> T + Send + Sync>,
    ) -> T {
        self.send_and_get_recv(f)
            .recv()
            .expect("background thread panicked")
    }

    #[inline]
    fn send_and_get_move<T: 'static + Send>(
        self,
        f: Box<dyn FnOnce(Stream) -> T + Send + Sync>,
    ) -> T {
        self.send_and_get_recv_move(f)
            .recv()
            .expect("background thread panicked")
    }

    /// Feed audio samples to an ongoing streaming inference.
    pub fn feed_audio(&self, buf: Vec<i16>) {
        self.send_and_get(Box::new(move |stream| stream.feed_audio(&buf[..])));
    }

    /// Compute the final decoding of an ongoing streaming inference and
    /// return the result.
    /// Signals the end of an ongoing streaming inference.
    ///
    /// Destroys this stream object.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    pub fn finish_stream(self) -> Result<String> {
        self.send_and_get_move(Box::new(Stream::finish_stream))
    }

    /// Compute the final decoding of an ongoing streaming inference
    /// and return results including metadata.
    /// Signals the end of an ongoing streaming inference.
    ///
    /// Destroys this stream object.
    ///
    /// `num_results` is the maximum number of possible transcriptions to return.
    /// Note that it is not guaranteed this many will be returned at minimum,
    /// but there will never be more than this number at maximum.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    pub fn finish_stream_with_metadata(self, num_results: u32) -> Result<Metadata> {
        self.send_and_get_move(Box::new(move |stream| {
            stream.finish_stream_with_metadata(num_results)
        }))
    }
}

#[cfg(feature = "async-streams")]
impl ThreadSafeStream {
    #[inline]
    async fn send_and_get_async<T: 'static + Send>(
        &self,
        f: Box<dyn FnOnce(&mut Stream) -> T + Send + Sync>,
    ) -> T {
        self.send_and_get_recv(f)
            .recv_async()
            .await
            .expect("background thread panicked")
    }

    #[inline]
    async fn send_and_get_move_async<T: 'static + Send>(
        self,
        f: Box<dyn FnOnce(Stream) -> T + Send + Sync>,
    ) -> T {
        self.send_and_get_recv_move(f)
            .recv_async()
            .await
            .expect("background thread panicked")
    }

    /// Asynchronously feed audio samples to an ongoing streaming inference.
    pub async fn feed_audio_async(&self, buf: Vec<i16>) {
        self.send_and_get_async(Box::new(move |stream| stream.feed_audio(&buf[..])))
            .await;
    }

    /// Asynchronously compute the final decoding of an ongoing streaming inference and
    /// return the result.
    /// Signals the end of an ongoing streaming inference.
    ///
    /// Destroys this stream object.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    pub async fn finish_stream_async(self) -> Result<String> {
        self.send_and_get_move_async(Box::new(Stream::finish_stream))
            .await
    }

    /// Asynchronously compute the final decoding of an ongoing streaming inference
    /// and return results including metadata.
    /// Signals the end of an ongoing streaming inference.
    ///
    /// Destroys this stream object.
    ///
    /// `num_results` is the maximum number of possible transcriptions to return.
    /// Note that it is not guaranteed this many will be returned at minimum,
    /// but there will never be more than this number at maximum.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    pub async fn finish_stream_with_metadata_async(self, num_results: u32) -> Result<Metadata> {
        self.send_and_get_move_async(Box::new(move |stream| {
            stream.finish_stream_with_metadata(num_results)
        }))
        .await
    }
}

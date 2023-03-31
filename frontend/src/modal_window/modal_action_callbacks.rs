use std::future::Future;
use std::pin::Pin;
use std::task::Poll::Ready;

use data_transport::DataSender;

pub type CallbackReturnType = Result<(), ()>;
pub type AsyncCallbackReturnType<'a> = Pin<Box<(dyn Future<Output = CallbackReturnType> + 'a)>>;
pub type ModalActionCallback<'a, T> =
    dyn Fn(&'a T, &'a mut DataSender<String>) -> AsyncCallbackReturnType<'a>;

pub struct DummyFuture {}

impl Future for DummyFuture {
    type Output = Result<(), ()>;

    fn poll(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Ready(Ok(()))
    }
}

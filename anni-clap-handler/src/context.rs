use std::any::Any;

#[derive(Default)]
pub struct Context {
    #[cfg(not(feature = "async"))]
    inner: Vec<Box<dyn Any>>,
    #[cfg(feature = "async")]
    inner: Vec<Box<dyn Any + Send>>,
}

impl Context {
    #[cfg(not(feature = "async"))]
    pub fn insert<T>(&mut self, param: T)
        where T: 'static {
        self.inner.push(Box::new(param));
    }

    #[cfg(feature = "async")]
    pub fn insert<T>(&mut self, param: T)
        where T: 'static + Send {
        self.inner.push(Box::new(param));
    }

    pub fn get<T>(&self) -> Option<&T>
        where T: 'static {
        // iterate from end to start
        for item in self.inner.iter().rev() {
            if let Some(data) = item.downcast_ref::<T>() {
                return Some(data);
            }
        }
        None
    }
}
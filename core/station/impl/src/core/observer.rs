use std::fmt::Debug;

pub type ObserverCallback<T> = Box<dyn Fn(&T) + Send + Sync>;

pub struct Observer<T> {
    listeners: Vec<ObserverCallback<T>>,
}

impl<T> Debug for Observer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Observer")
            .field("listeners", &self.listeners.len())
            .finish()
    }
}

impl<T> Default for Observer<T> {
    fn default() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
}

impl<T> Observer<T> {
    pub fn add_listener(&mut self, listener: ObserverCallback<T>) {
        self.listeners.push(listener);
    }

    pub fn notify(&self, t: &T) {
        for listener in &self.listeners {
            listener(t);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    thread_local! {
        static COUNT: RefCell<i32> = const { RefCell::new(0) };
    }

    #[test]
    fn test_observer() {
        let mut observer = Observer::default();
        observer.add_listener(Box::new(|t| {
            assert_eq!(*t, 42);
            COUNT.with(|count| {
                *count.borrow_mut() += 1;
            });
        }));
        observer.add_listener(Box::new(|t| {
            assert_eq!(*t, 42);
            COUNT.with(|count| {
                *count.borrow_mut() += 1;
            });
        }));

        observer.notify(&42);

        COUNT.with(|count| {
            assert_eq!(*count.borrow(), 2);
        });
    }

    #[test]
    fn test_observer_debug() {
        let observer = Observer::<i32>::default();

        assert_eq!(format!("{:?}", observer), "Observer { listeners: 0 }");
    }
}

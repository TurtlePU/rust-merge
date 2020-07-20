pub trait Mergeable<F, T, I> {
    fn merge(self, f: F) -> Merge<F, T, I>;
}

impl<F, T, I> Mergeable<F, T, I> for I where I: Sized {
    fn merge(self, f: F) -> Merge<F, T, I> {
        Merge { merger: f, accum: None, iter: self }
    }
}

pub struct Merge<F, T, I> {
    merger: F,
    accum: Option<T>,
    iter: I,
}

impl<F, T, I> Iterator for Merge<F, T, I>
where F: FnMut(T, T) -> Result<T, (T, T)>, I: Iterator<Item = T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if let Some(accum) = self.accum.take() {
                match (self.merger)(accum, item) {
                    Ok(accum) => {
                        self.accum = Some(accum);
                    },
                    Err((result, item)) => {
                        self.accum = Some(item);
                        return Some(result);
                    },
                }
            } else {
                self.accum = Some(item);
            }
        }
        self.accum.take()
    }
}

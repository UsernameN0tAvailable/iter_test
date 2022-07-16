use std::{mem, sync::Arc, fmt::Display};


const C_SIZE: usize = 10;

pub trait HasLength {
    fn len(&self) -> usize;
}

impl<T> HasLength for &Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl<T> HasLength for &[T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

struct ParChunker<I: Iterator, F> {
    iter: I,
    chunk: Vec<I::Item>,
    //max_total_size: usize,
    chunk_size: usize,
    total_size: usize,
    f: F
}

impl<I, F, R> Iterator for ParChunker<I, F>
where
    I: Iterator + Send,
    I::Item: Send + Display + Copy,
    F: Fn(I::Item) -> R + Send + Sync,
    R: Send
{
    type Item = Vec<I::Item>;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(item) => {  
                    (self.f)(item);
                    self.total_size += 1;
                    self.chunk.push(item);
                    //println!("{}", item.len());
 
                    if self.total_size >= self.chunk_size {
                        self.total_size = 0;
                        return Some(mem::take(&mut self.chunk))
                    }
                }
                None => return if self.chunk.is_empty() {
                    None
                } else {
                    Some(mem::take(&mut self.chunk))
                }
            }
        }
    }
}

trait ChunkExt <I, F, R>: Iterator + Sized where 
    I: Iterator,
    F: Fn(I::Item) -> R + Sync + Send + Sized,
    R: Sync + Send
{
    fn par_chunks(self, f: F) -> ParChunker<Self, F> {
        ParChunker {
            iter: self,
            chunk: Vec::new(), 
            chunk_size: C_SIZE,
            total_size: 0,
            f
        }
    }
}

impl<I: Iterator, F: Fn(I::Item) -> R + Send + Sync, R: Send + Sync> ChunkExt<I, F, R> for I {}


fn main() {

    let v: Vec<i32> = (0..453).map(|i| i).collect();
    let a: Arc<[i32]> = Arc::from(v);

    let c = a
        .iter()
        .par_chunks(|a| println!("{:?}", a));

    let t: Vec<Vec<&i32>> = c.collect();

    println!("{:?}", t);



    //for g in c {}

    //i.next();


    //println!("{}", a);
}

use std::sync::Arc;



const THREAD_N: usize = 8;


pub struct ChunkIter<'a, T, F, R>
where 
    T: Sized + Send + Sync, 
    F: Fn(&[T]) ->  R + Send + Sync,
    R: Sized + Send + Sync,
     // R: Sized + Send + Sync 
{
    values: &'a Arc<[T]>,
    start_index: usize,
    end_index: usize,
    chunk_size: usize,
    f: F
}


impl<'a, T: Sized + Send + Sync, F: Fn(&[T]) -> R + Send + Sync, R: Sized + Send + Sync> Iterator for ChunkIter<'a, T, F, R> {
    type Item = R;


    fn next(&mut self) -> Option<Self::Item> {

        loop {

            let c_size: usize  = self.end_index - self.start_index; 


            //(self.f)()

            if self.end_index < self.values.len() {       

                if c_size < self.chunk_size { 
                    self.end_index += 1;
                } else {
                    let out = Some((self.f)(&self.values[self.start_index..self.end_index]));
                    self.start_index = self.end_index;
                    return out;
                }

            } else if c_size > 0 {
                let out = Some((self.f)(&self.values[self.start_index..self.end_index]));
                self.start_index = self.end_index;
                return out;
            } else {
                return None
            }
        }
    }
}

pub trait IntoChunkIter<'a, T: Sized + Send + Sync, F: Fn(&[T]) -> R + Send + Sync, R: Sized + Send + Sync >{
    fn into_chunk_iter(&'a self, f: F) ->  ChunkIter<'a, T, F, R>;
}


impl<'a, T: Send + Sync, F: Fn(&[T]) -> R + Send + Sync, R: Sized + Send + Sync> IntoChunkIter<'a, T, F, R> for Arc<[T]> {

    fn into_chunk_iter(&'a self, f: F) ->  ChunkIter<'a, T, F, R> {     
        ChunkIter {
            chunk_size: self.len() / THREAD_N + 1, 
            values: self,
            start_index: 0,
            end_index: 0,
            f
        }
    }

} 




fn main() {

    let v: Vec<i32> = (0..20).collect();

    let b: Arc<[i32]> = Arc::from(v);

    for _t in b.into_chunk_iter(|a| {
        println!("{:?}", a);
        10
    }) {}


    //b.into_

    /*
    for a in v.into_chunk_iter() {
        println!("{:?}", a);
    } */

}

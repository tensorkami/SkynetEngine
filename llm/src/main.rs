
struct tensor{

    data : *mut i32,
    size : usize,
    ndims: usize,
    shape : Vec<u8>,
    stride: Vec<u8>

}

fn mm(m:usize, n:usize, k:usize, A: *const i32, B: *const i32, c: *mut i32) {

  let mut tmp = 0;
unsafe{
for i in 0..m{
  for j in 0..n{
    for z in 0..k{
      tmp += *A.add(i * k + z) * *B.add(j + n*z);
    }
    *c.add(i*n + j) = tmp;
    tmp = 0;

  }
}
}
}




impl tensor{

  fn   get(&self, index : Vec<u8>) -> *mut i32{
    let mut ind:usize = 0;
    for i in 0..self.ndims{
    ind +=(self.stride[i] * index[i]) as usize;

    }

    unsafe{ self.data.add(ind)}

  }

  fn view(&self, resh : Vec<u8>) -> Self{
    let dim = resh.len();
    let mut stride_new: Vec<u8> = vec![0;dim];
    stride_new[dim -1] = 1;
    for i in 1..dim{

      stride_new[dim - 1 - i] = resh[dim  - i] *  stride_new[dim - i];
    }

      Self{data : self.data,
        size : self.size,
        ndims : dim,
        shape : resh,
        stride : stride_new

        }
  }
    
} 


fn main() {

   let mut v:Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18];

    let mut s  = tensor{data: v.as_mut_ptr(), shape : vec![2,9] ,size: 0,ndims : 2,  stride : vec![9,1]};
    unsafe {*(s.get(vec![1,1])) = 56;}


let s1 = s.view(vec![2,3,3]);
unsafe {*(s1.get(vec![1,1,1])) = 567;}





  let mut va:Vec<i32> = vec![1,2,3,
                             4,5,6];
  let mut vb:Vec<i32> = vec![3,4,5,1,
                            2,1,2,1,
                            2,1,2,1];
  let mut vc:Vec<i32> = vec![0,0,0,0,0,0,0,0];
unsafe{mm(2,4,3, va.as_ptr(), vb.as_ptr(), vc.as_mut_ptr());}

for i in vc{
  println!("{}", i);
}

   
}

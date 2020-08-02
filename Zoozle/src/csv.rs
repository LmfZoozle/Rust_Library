//csv.rs

pub struct csv_ex{
    pub sheet:Vec<Vec<String>>,
    pub line :Vec<String>, 
    pub ylength:i32,
    pub xlength:i32,
   } 

    impl csv_ex{
     pub  fn new(path:String)->Self{
           let contents = std::fs::read_to_string(path)
           .expect("From Csv_Ex : Failed read file");
           let mut line = 0;
           

           for run in contents.chars(){
               if '\n'==run {
                   line+=1;
               }else if '\r'==run {
                   line+=1;
               }
           }

           let mut max_x:Vec<i32>=Vec::with_capacity(line);
           max_x.push(1);

           let mut max_now=0;
           let mut xvalue=1;
           for run in contents.chars(){
               match run{
                   '\n'=>{
                       max_now+=1;
                       max_x.push(1);
                       xvalue+=1;
                   },
                   '\r'=>{
                       max_now+=1;
                       max_x.push(1);
                       xvalue+=1;
                   },
                   ','=>{
                       max_x[max_now]+=1;
                       xvalue+=1;
                   }
                   _=>{
           
                   },
               } 
           }

           let mut temp1=max_x[0];
           let mut ii=0;

           while ii<line{
               if temp1<max_x[ii]{
                super::oft::swap(&mut temp1,&mut max_x[ii])
               }
               ii+=1;
            }

           let mut origin:Vec<String>=Vec::with_capacity(xvalue);
           println!("xlen={} , ylen={}",temp1,line);
//            panic!("がんばれよ");
           let mut buffer=String::new();
           for run in contents.chars(){
               match run{
                   '\n'=>{
                       origin.push(buffer.clone());
                       buffer.clear();
                   },
                   '\r'=>{
                       origin.push(buffer.clone());
                       buffer.clear();
                   },
                   ','=>{
                       origin.push(buffer.clone());
                       buffer.clear();
                   }
                   _=>{
                       buffer.push(run);            
                   },
               }
           }

           let mut newsheet:Vec<Vec<String>>=Vec::with_capacity(line);
           unsafe{
               newsheet.set_len(line);
           }

           let mut count=0;
           let mut buffer=String::new();
           for run in contents.chars(){
               match run{
                   '\n'=>{
                       newsheet[count].push(buffer.clone());
                       buffer.clear();
                       count+=1;
                   },
                   '\r'=>{
                       newsheet[count].push(buffer.clone());
                       buffer.clear();
                       count+=1;
                   },
                   ','=>{
                     newsheet[count].push(buffer.clone());
                     buffer.clear();
                   }
                   _=>{
                     buffer.push(run);
                   },
               }
           }


           csv_ex{
               line:origin,
               sheet:newsheet,
               ylength:line as i32,
               xlength:0,
           }
       }

       pub fn mesh_able(&self,x:i32,y:i32)->bool{
           if self.ylength<=y{
               false;
           }
           match self.sheet[y as usize].get(x as usize){
             Some(_)=>{
               true
             },
             None=>{
               false
             },
           }
       }

   }
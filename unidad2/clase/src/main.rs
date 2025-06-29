fn main() {
        let edad_a: i32 = 28;
        let edad_b: u16 = 30;
        
        if edad_a < (edad_b as i32) {
                println!("edadA es menor que edadB");
        }
        
        ////////////////////////////////////////////
        
        let value: i32 = 1500;
        let result: i8 = value as i8;
        print!("result: {}", result);
        
        ///////////////////////////////////////////
        
        let value: i32 = 1500;
        let result: i16 = value.try_into().unwrap();
        println!("result: {}", result);
        
        ////////////////////////////////////////////
        
        let mut cont: i32 = 0;
        cont = cont + 2;

        {
                let mut cont:i32 = cont * 5;
                cont = cont + 1;
                println!("valor del alcance interno {}", cont);
        }
        
        println!("valor orginal de cont: {}", cont);
        
        
        ///////////////////////////////////////////////////
        
        let tupla: (i32, f64, u8) = (1501, 7.2, 5);
        
        let tupla_2 = (1500, 7.1, 1);
        
        let (x, y, z) = tupla_2;
        
        println!("x: {}, y: {}, z: {}", x, y, z);
        
        println!("tupla: {:?}", tupla);
        
        println!("x: {}", tupla.0);
        println!("y: {}", tupla.1);
        println!("z: {}", tupla.2);
        
        
        
}

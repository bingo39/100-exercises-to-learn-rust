// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.


//翻译：
/* 它应该跟踪三个信息：`product_name`（产品名称）、`quantity`（数量）和 `unit_price`（单价）。
   产品名称不能为空，并且不能超过 300 字节。
   数量必须严格大于零。
   单价以分为单位，必须严格大于零。
   Order 必须包含一个名为 `total` 的方法，返回订单的总价。
   Order 必须为每个字段提供 setter 和 getter 方法。

 这次测试位于不同的地方——在 `tests` 文件夹中。
 `tests` 文件夹对于 `cargo` 来说是一个特殊位置。它是 `cargo` 查找**集成测试**的地方。
 这里的"集成"有一个非常具体的含义：它们测试你项目的**公共 API**。
 你需要注意类型和方法的可见性；集成测试无法访问私有项或 `pub(crate)` 项。
 */

 pub struct Order{
  product_name:String,
  quantity:u32,
  unit_price:u32,
 }

 impl Order {
  //构造函数
  pub fn new(product_name:String,quantity:u32,unit_price:u32)->Order{
    Order::vaild_product_name(&product_name);
    Order::vaild_quantity(quantity);
    Order::vaild_unit_price(unit_price);
    //声明引用
    Order { 
      product_name, 
      quantity, 
      unit_price 
    }
  }
    // setter
    pub fn set_product_name(&mut self,product_name:String){
        Order::vaild_product_name(&product_name);
        self.product_name = product_name;
    }
    pub fn set_quantity(&mut self,quantity:u32){
      Order::vaild_quantity(quantity);
        self.quantity = quantity;
    }
    pub fn set_unit_price(&mut self,unit_price:u32){
      Order::vaild_unit_price(unit_price);
        self.unit_price = unit_price;
    }
    // getter
    pub fn product_name(&self)->&str{
        &self.product_name
    }
    pub fn quantity(&self)->&u32{
        &self.quantity
    }
    pub fn unit_price(&self)->&u32{
        &self.unit_price
    }


     fn vaild_product_name(product_name:&String) {
         if product_name.is_empty(){
          panic!("product_name cannot be empty");
         }
         if product_name.len() > 300{
          panic!("product_name cannot be longer than 300 bytes");
         }
      }
      // 根据传参原则，这里设计u32而不是&u32
      fn vaild_quantity(quantity:u32) {
        if quantity == 0{
          panic!("quantity cannot greater than zero");
        }
      }
      fn vaild_unit_price(unit_price:u32) {
          if unit_price == 0{
            panic!("unit_price cannot greater than zero")
          }
      }
      pub fn total(&self)->u32{
        &self.quantity * &self.unit_price
      }
  }
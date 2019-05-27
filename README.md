NOTE: This is a snippet of Rust Futures 0.3 wrapper code that I cannot get working...

```
error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
  --> src/main.rs:10:23
   |
10 |         Box::new(self.inner().boxed().compat())
   |                       ^^^^^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 9:5...
  --> src/main.rs:9:5
   |
9  | /     fn wrapper(&self) -> Box<Future<Item = u8, Error = String>> {
10 | |         Box::new(self.inner().boxed().compat())
11 | |     }
   | |_____^
note: ...so that reference does not outlive borrowed content
  --> src/main.rs:10:18
   |
10 |         Box::new(self.inner().boxed().compat())
   |                  ^^^^
   = note: but, the lifetime must be valid for the static lifetime...
   = note: ...so that the expression is assignable:
           expected std::boxed::Box<(dyn futures::future::Future<Error = std::string::String, Item = u8> + 'static)>
              found std::boxed::Box<dyn futures::future::Future<Error = std::string::String, Item = u8>>
```

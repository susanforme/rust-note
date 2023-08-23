/*
 * @Author: susanforme
 * @Date: 2023-08-23 16:45:59
 * @LastEditTime: 2023-08-23 17:14:33
 * @FilePath: \rust-note\src\marco.rs
 * @Description:
 */
#[macro_export]
/// 枚举
macro_rules! enum_str {
  (enum $name:ident {
      $($variant:ident),*,
  }) => {
      enum $name {
          $($variant ),*
      }

      impl $name {
          fn name(&self) -> &'static str {
              match self {
                  $($name::$variant => stringify!($variant)),*
              }
          }
      }
  };
}
#[macro_export]
macro_rules! pub_enum_str {
  (enum $name:ident {
      $($variant:ident),*,
  }) => {
     pub enum $name {
          $($variant ),*
      }

      impl $name {
        /// return self name
         pub fn name(&self) -> &'static str {
              match self {
                  $($name::$variant => stringify!($variant)),*
              }
          }
      }
  };
}

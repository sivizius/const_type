/// With `Const!` `enum`-like `const`-types, but with aliases for variants, can be defined.
#[macro_export]
macro_rules! Const
{
  (
    $(  #[  $Config:meta ]  )*
    $Visibility:vis $Struct:ident: $Type:ty
    {
      $(
        $(  #[  $Variant:meta ]  )*
        $Const:ident                 =   $Value:expr$(,)*
      )*
    }
  )
  =>  {
        $(  #[  $Config ]  )*
        #[derive(Copy,Clone,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
        $Visibility struct  $Struct     ( $Type );
        impl $Struct
        {
          $(
            $(  #[  $Variant ]  )*
            pub const $Const: $Struct   =   $Struct ( $Value  );
          )*
        }
      };
  (
    $(  #[  $Config:meta ]  )*
    $Visibility:vis $Struct:ident
    {
      $(
        $(  #[  $Variant:meta ]  )*
        $Const:ident                    =   $Value:expr$(,)*
      )*
    }
  )
  =>  {
        $(  #[  $Config ]  )*
        #[derive(Copy,Clone,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
        $Visibility struct  $Struct     ( usize );
        impl $Struct
        {
          $(
            $(  #[  $Variant ]  )*
            pub const $Const: $Struct   =   $Struct ( $Value  );
          )*
        }
      };
}

#[macro_export]
macro_rules! Const
{
  (
    $Visibility:vis $NameSpace:ident: $Type:ty
    {
      $(  $Constant:ident               =   $Value:expr$(,)*    )*
    }
  ) =>  (
          #[derive(Copy,Clone,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
          $Visibility struct  $NameSpace  ( $Type );
          impl $NameSpace
          {
            $(
              pub const
              $Constant:      $NameSpace
                                        =   $NameSpace  ( $Value  );
            )*
          }
        );
  (
    $Visibility:vis $NameSpace:ident
    {
      $(  $Constant:ident               =   $Value:expr$(,)*    )*
    }
  ) =>  (
          #[derive(Copy,Clone,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
          $Visibility struct  $NameSpace  ( usize );
          impl $NameSpace
          {
            $(
              pub const
              $Constant:      $NameSpace
                                        =   $NameSpace  ( $Value  );
            )*
          }
        );
}

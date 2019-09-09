#[macro_export]
macro_rules! Const
{
  (
    $Visibility:vis $NameSpace:ident: $Type:ty
    {
      $(  $Constant:ident               =   $Value:expr$(,)*    )*
    }
  ) =>  (#[derive(Copy,Clone,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
          $Visibility struct  $NameSpace  ( pub $Type );
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
  ) =>  (#[derive(Copy,Clone,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
          $Visibility struct  $NameSpace  ( pub usize );
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

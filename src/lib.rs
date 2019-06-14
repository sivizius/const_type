#[macro_export]
macro_rules! Const
{
  (
    $Visibility:vis $NameSpace:ident: $Type:ty
    {
      $(  $FirstConstant:ident          =   $FirstValue:expr )?
      $(  , $Constant:ident             =   $Value:expr    )*$(,)?
    }
  ) =>  (
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
}

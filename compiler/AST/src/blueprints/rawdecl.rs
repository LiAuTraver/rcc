#[derive(Debug, Clone, Copy, ::strum_macros::Display)]
pub enum VarDeclKind {
  /// declaration:
  ///   - file-scope: without initializer, with storage-class specifier(`extern`/`static`)
  ///   - block-scope: vardecl without initializer, with `extern` specifier (initializer is not allowed); functiondecl
  Declaration,
  /// complete definition.
  ///   - file-scope: with initializer, regardless of the presence of storage-class specifier
  ///   - block-scope: variable declaration without `extern` specifier
  Definition,
  /// 6.9.3p2: A declaration of an identifier for an object that has file scope without an initializer, and without
  /// the storage-class specifier `extern` or `thread_local`, constitutes a *tentative definition*.
  ///
  /// ```c
  /// int a; // tentative definition
  /// extern int a; // declaration
  /// int a = 0; // complete definition
  /// static int a; // ok, still tentative definition
  /// extern int a; // ok, still declaration
  /// // int a = 1; // error: redefinition
  /// ```
  ///
  /// if no complete definition is found, the tentative definition is treated as a complete definition with empty initializer.
  ///
  /// - if the composite type as of the end of the translation unit is an array of unknown size,
  ///   then an array of size one with the composite element type;
  /// - otherwise, the composite type \[...].
  ///
  /// if it has internal linkage, the type shall be complete.
  Tentative,
}
impl VarDeclKind {
  pub fn merge(lhs: Self, rhs: Self) -> Self {
    use VarDeclKind::*;
    match (lhs, rhs) {
      (Tentative, Tentative) => Tentative,
      (Definition, _) | (_, Definition) => Definition,
      _ => Declaration,
    }
  }
}

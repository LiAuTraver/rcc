use ::rcc_sema::{
  declaration::{
    ExternalDeclaration, Function, Initializer, TranslationUnit, VarDef,
  },
  expression::Expression,
  statement::Statement,
};

use crate::{DumpSpan, Dumpable, Dumper, Palette, quoted};

impl<'c> Dumpable<'c> for Expression<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    use ::rcc_sema::expression::{RawExpr::*, *};

    dumper.print_indent(prefix, is_last);
    macro_rules! header {
      ($name:expr, $raw:ident) => {
        header!($name, $raw, "")
      };
      ($name:expr, $raw:ident, $newline:literal) => {
        dumper.write($name, &palette.node);
        dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim);
        $raw.span.dump(dumper, prefix, is_last, palette);
        dumper.write_fmt(
          format_args!("'{}' ", self.qualified_type()),
          &palette.meta,
        );
        dumper.write_fmt(
          format_args!(concat!("{} ", $newline), self.value_category()),
          &palette.info,
        );
      };
    }

    let subprefix = dumper.child_prefix(prefix, is_last);

    match self.raw_expr() {
      Empty(_) => dumper.write("<<<Recovery/Invalid>>>\n", &palette.error),

      Constant(constant) => {
        dumper.write("ConstantLiteral", &palette.node);
        dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim);
        constant.span.dump(dumper, prefix, is_last, palette);
        dumper.write_fmt(
          format_args!("'{}' ", self.qualified_type()),
          &palette.meta,
        );
        // didnt print RValue.
        dumper.write_fmt(format_args!("{}\n", constant.inner), &palette.literal)
      },

      Variable(variable) => {
        header!("Variable", variable);
        dumper.write_fmt(
          format_args!(" '{}'\n", variable.name.borrow()),
          &palette.literal,
        )
      },

      Unary(unary) => {
        header!("Unary", unary);
        dumper.write_fmt(
          format_args!(" {} '{}'\n", unary.kind, unary.operator),
          &palette.operator,
        );
        // One child: the operand (it's the last child)
        unary.operand.dump(dumper, &subprefix, true, palette)
      },

      Binary(binary) => {
        header!("Binary", binary);
        dumper.write_fmt(
          format_args!(" '{}'\n", binary.operator),
          &palette.operator,
        );
        // Two children: left (not last), right (last)
        binary.left.dump(dumper, &subprefix, false, palette);
        binary.right.dump(dumper, &subprefix, true, palette)
      },

      Ternary(ternary) => {
        header!("Ternary", ternary, "\n");
        ternary.condition.dump(dumper, &subprefix, false, palette);
        ternary.then_expr.dump(dumper, &subprefix, false, palette);
        ternary.else_expr.dump(dumper, &subprefix, true, palette)
      },

      Call(call) => {
        header!("Call", call, "\n");
        // callee + N arguments
        let total = 1 + call.arguments.len();
        call.callee.dump(dumper, &subprefix, total == 1, palette);
        for (i, arg) in call.arguments.iter().enumerate() {
          arg.dump(dumper, &subprefix, i == call.arguments.len() - 1, palette);
        }
      },

      Paren(paren) => {
        header!("Paren", paren, "\n");
        paren.expr.dump(dumper, &subprefix, true, palette)
      },

      ImplicitCast(cast) => {
        header!("ImplicitCast", cast);
        dumper.write(" <", &palette.skeleton);
        dumper.write(cast.cast_type, &palette.kind);
        dumper.write(">\n", &palette.skeleton);
        cast.expr.dump(dumper, &subprefix, true, palette)
      },

      MemberAccess(ma) => {
        header!("MemberAccess", ma);
        dumper.write_fmt(format_args!(" .{}\n", ma.member), &palette.literal);
        ma.object.dump(dumper, &subprefix, true, palette)
      },

      ArraySubscript(sub) => {
        header!("ArraySubscript", sub, "\n");
        sub.array.dump(dumper, &subprefix, false, palette);
        sub.index.dump(dumper, &subprefix, true, palette)
      },

      SizeOf(sof) => {
        header!("SizeOf", sof, "\n");
        match &sof.sizeof {
          SizeOfKind::Type(ty) => {
            // // Type child — just print it inline (no recursion needed)
            // dumper.print_indent(&subprefix, true);
            // dumper.write_fmt(format_args!("Type '{}'\n", ty), &palette.meta)

            dumper.print_indent(&subprefix, true);
            ty.dump(dumper, prefix, true, palette)
          },
          SizeOfKind::Expression(expr) =>
            expr.dump(dumper, &subprefix, true, palette),
        }
      },

      CStyleCast(cast) => {
        header!("CStyleCast", cast);
        // dumper.write_fmt(
        //   format_args!(" '{}'\n", cast.target_type),
        //   &palette.meta,
        // );
        cast.expr.dump(dumper, &subprefix, true, palette)
      },

      CompoundLiteral(cl) => {
        header!("CompoundLiteral", cl, "\n");
      },
    }
  }
}
impl<'c> Dumpable<'c> for TranslationUnit<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    dumper.write("TranslationUnit", &palette.node);
    dumper.write_fmt(format_args!(" {:p}\n", self), &palette.dim);
    let subprefix = dumper.child_prefix(prefix, is_last);
    self.declarations.iter().enumerate().for_each(|(i, decl)| {
      decl.dump(
        dumper,
        &subprefix,
        i == self.declarations.len() - 1,
        palette,
      )
    });
  }
}
impl<'c> Dumpable<'c> for ExternalDeclaration<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    ::rcc_utils::static_dispatch!(
      self,
      |variant| variant.dump(dumper, prefix, is_last, palette) =>
      Variable Function
    )
  }
}

impl<'c> Dumpable<'c> for VarDef<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    dumper.print_indent(prefix, is_last);
    let borrowed = self.symbol.borrow();
    dumper.write(
      if borrowed.is_typedef() {
        "Typedef"
      } else {
        "VarDef"
      },
      &palette.node,
    );
    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim);
    self.span.dump(dumper, prefix, is_last, palette);

    dumper.write("<", &palette.skeleton);
    dumper.write(borrowed.declkind, &palette.kind);
    dumper.write(">", &palette.skeleton);

    dumper.write_fmt(format_args!(" '{}' ", borrowed.name), &palette.literal);

    dumper.write("[", &palette.skeleton);
    dumper
      .write_fmt(format_args!("'{}'", borrowed.qualified_type), &palette.meta);

    dumper.write_fmt(
      format_args!(" {:p}", borrowed.qualified_type.unqualified_type),
      &palette.skeleton,
    );
    dumper.write("]\n", &palette.skeleton);

    if let Some(initializer) = &self.initializer {
      let subprefix = dumper.child_prefix(prefix, is_last);
      initializer.dump(dumper, &subprefix, true, palette);
    }
  }
}
impl<'c> Dumpable<'c> for Function<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    dumper.print_indent(prefix, is_last);
    dumper.write("Function", &palette.node);
    dumper.write_fmt(format_args!(" {:p}", self), &palette.dim);
    self.span.dump(dumper, prefix, is_last, palette);

    dumper.write("<", &palette.skeleton);
    dumper.write(self.symbol.borrow().declkind, &palette.kind);
    dumper.write(">", &palette.skeleton);
    dumper.write_fmt(
      quoted!(" '", self.symbol.borrow().name, "' "),
      &palette.literal,
    );
    dumper.write("[", &palette.skeleton);
    dumper.write(
      quoted!("'" => self.symbol.borrow().qualified_type),
      &palette.meta,
    );
    dumper.write_fmt(
      format_args!(
        " {:p}",
        self.symbol.borrow().qualified_type.unqualified_type
      ),
      &palette.skeleton,
    );
    dumper.write("]\n", &palette.skeleton);

    if let Some(body) = &self.body {
      let subprefix = dumper.child_prefix(prefix, is_last);
      body.dump(dumper, &subprefix, true, palette);
    }
  }
}

impl<'c> Dumpable<'c> for Initializer<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    dumper.print_indent(prefix, is_last);
    dumper.write("Initializer", &palette.node);
    match self {
      Self::Scalar(expression) => {
        dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim);
        expression.span().dump(dumper, prefix, is_last, palette);
        dumper.newline();
        let subprefix = dumper.child_prefix(prefix, is_last);
        expression.dump(dumper, &subprefix, true, palette)
      },
      Self::Aggregate(_) => todo!(),
    }
  }
}

impl<'c> Dumpable<'c> for Statement<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    ::rcc_utils::static_dispatch!(
      self,
      |variant| variant.dump(dumper, prefix, is_last, palette) =>
      Empty Return Expression Declaration Compound If While DoWhile For Switch Goto Label Break Continue
    )
  }
}

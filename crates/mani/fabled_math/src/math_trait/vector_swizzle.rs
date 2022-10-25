pub trait Swizzles2: Sized + Copy + Clone {
    type Swizzle3;

    type Swizzle4;

    #[inline]
    fn xy(self) -> Self {
        self
    }

    fn xx(self) -> Self;

    fn yx(self) -> Self;

    fn yy(self) -> Self;

    fn xxx(self) -> Self::Swizzle3;

    fn xxy(self) -> Self::Swizzle3;

    fn xyx(self) -> Self::Swizzle3;

    fn xyy(self) -> Self::Swizzle3;

    fn yxx(self) -> Self::Swizzle3;

    fn yxy(self) -> Self::Swizzle3;

    fn yyx(self) -> Self::Swizzle3;

    fn yyy(self) -> Self::Swizzle3;

    fn xxxx(self) -> Self::Swizzle4;

    fn xxxy(self) -> Self::Swizzle4;

    fn xxyx(self) -> Self::Swizzle4;

    fn xxyy(self) -> Self::Swizzle4;

    fn xyxx(self) -> Self::Swizzle4;

    fn xyxy(self) -> Self::Swizzle4;

    fn xyyx(self) -> Self::Swizzle4;

    fn xyyy(self) -> Self::Swizzle4;

    fn yxxx(self) -> Self::Swizzle4;

    fn yxxy(self) -> Self::Swizzle4;

    fn yxyx(self) -> Self::Swizzle4;

    fn yxyy(self) -> Self::Swizzle4;

    fn yyxx(self) -> Self::Swizzle4;

    fn yyxy(self) -> Self::Swizzle4;

    fn yyyx(self) -> Self::Swizzle4;

    fn yyyy(self) -> Self::Swizzle4;
}

pub trait Swizzles3: Sized + Copy + Clone {
    type Swizzle2;

    type Swizzle4;

    #[inline]
    fn xyz(self) -> Self {
        self
    }

    fn xx(self) -> Self::Swizzle2;

    fn xy(self) -> Self::Swizzle2;

    fn xz(self) -> Self::Swizzle2;

    fn yx(self) -> Self::Swizzle2;

    fn yy(self) -> Self::Swizzle2;

    fn yz(self) -> Self::Swizzle2;

    fn zx(self) -> Self::Swizzle2;

    fn zy(self) -> Self::Swizzle2;

    fn zz(self) -> Self::Swizzle2;

    fn xxx(self) -> Self;

    fn xxy(self) -> Self;

    fn xxz(self) -> Self;

    fn xyx(self) -> Self;

    fn xyy(self) -> Self;

    fn xzx(self) -> Self;

    fn xzy(self) -> Self;

    fn xzz(self) -> Self;

    fn yxx(self) -> Self;

    fn yxy(self) -> Self;

    fn yxz(self) -> Self;

    fn yyx(self) -> Self;

    fn yyy(self) -> Self;

    fn yyz(self) -> Self;

    fn yzx(self) -> Self;

    fn yzy(self) -> Self;

    fn yzz(self) -> Self;

    fn zxx(self) -> Self;

    fn zxy(self) -> Self;

    fn zxz(self) -> Self;

    fn zyx(self) -> Self;

    fn zyy(self) -> Self;

    fn zyz(self) -> Self;

    fn zzx(self) -> Self;

    fn zzy(self) -> Self;

    fn zzz(self) -> Self;

    fn xxxx(self) -> Self::Swizzle4;

    fn xxxy(self) -> Self::Swizzle4;

    fn xxxz(self) -> Self::Swizzle4;

    fn xxyx(self) -> Self::Swizzle4;

    fn xxyy(self) -> Self::Swizzle4;

    fn xxyz(self) -> Self::Swizzle4;

    fn xxzx(self) -> Self::Swizzle4;

    fn xxzy(self) -> Self::Swizzle4;

    fn xxzz(self) -> Self::Swizzle4;

    fn xyxx(self) -> Self::Swizzle4;

    fn xyxy(self) -> Self::Swizzle4;

    fn xyxz(self) -> Self::Swizzle4;

    fn xyyx(self) -> Self::Swizzle4;

    fn xyyy(self) -> Self::Swizzle4;

    fn xyyz(self) -> Self::Swizzle4;

    fn xyzx(self) -> Self::Swizzle4;

    fn xyzy(self) -> Self::Swizzle4;

    fn xyzz(self) -> Self::Swizzle4;

    fn xzxx(self) -> Self::Swizzle4;

    fn xzxy(self) -> Self::Swizzle4;

    fn xzxz(self) -> Self::Swizzle4;

    fn xzyx(self) -> Self::Swizzle4;

    fn xzyy(self) -> Self::Swizzle4;

    fn xzyz(self) -> Self::Swizzle4;

    fn xzzx(self) -> Self::Swizzle4;

    fn xzzy(self) -> Self::Swizzle4;

    fn xzzz(self) -> Self::Swizzle4;

    fn yxxx(self) -> Self::Swizzle4;

    fn yxxy(self) -> Self::Swizzle4;

    fn yxxz(self) -> Self::Swizzle4;

    fn yxyx(self) -> Self::Swizzle4;

    fn yxyy(self) -> Self::Swizzle4;

    fn yxyz(self) -> Self::Swizzle4;

    fn yxzx(self) -> Self::Swizzle4;

    fn yxzy(self) -> Self::Swizzle4;

    fn yxzz(self) -> Self::Swizzle4;

    fn yyxx(self) -> Self::Swizzle4;

    fn yyxy(self) -> Self::Swizzle4;

    fn yyxz(self) -> Self::Swizzle4;

    fn yyyx(self) -> Self::Swizzle4;

    fn yyyy(self) -> Self::Swizzle4;

    fn yyyz(self) -> Self::Swizzle4;

    fn yyzx(self) -> Self::Swizzle4;

    fn yyzy(self) -> Self::Swizzle4;

    fn yyzz(self) -> Self::Swizzle4;

    fn yzxx(self) -> Self::Swizzle4;

    fn yzxy(self) -> Self::Swizzle4;

    fn yzxz(self) -> Self::Swizzle4;

    fn yzyx(self) -> Self::Swizzle4;

    fn yzyy(self) -> Self::Swizzle4;

    fn yzyz(self) -> Self::Swizzle4;

    fn yzzx(self) -> Self::Swizzle4;

    fn yzzy(self) -> Self::Swizzle4;

    fn yzzz(self) -> Self::Swizzle4;

    fn zxxx(self) -> Self::Swizzle4;

    fn zxxy(self) -> Self::Swizzle4;

    fn zxxz(self) -> Self::Swizzle4;

    fn zxyx(self) -> Self::Swizzle4;

    fn zxyy(self) -> Self::Swizzle4;

    fn zxyz(self) -> Self::Swizzle4;

    fn zxzx(self) -> Self::Swizzle4;

    fn zxzy(self) -> Self::Swizzle4;

    fn zxzz(self) -> Self::Swizzle4;

    fn zyxx(self) -> Self::Swizzle4;

    fn zyxy(self) -> Self::Swizzle4;

    fn zyxz(self) -> Self::Swizzle4;

    fn zyyx(self) -> Self::Swizzle4;

    fn zyyy(self) -> Self::Swizzle4;

    fn zyyz(self) -> Self::Swizzle4;

    fn zyzx(self) -> Self::Swizzle4;

    fn zyzy(self) -> Self::Swizzle4;

    fn zyzz(self) -> Self::Swizzle4;

    fn zzxx(self) -> Self::Swizzle4;

    fn zzxy(self) -> Self::Swizzle4;

    fn zzxz(self) -> Self::Swizzle4;

    fn zzyx(self) -> Self::Swizzle4;

    fn zzyy(self) -> Self::Swizzle4;

    fn zzyz(self) -> Self::Swizzle4;

    fn zzzx(self) -> Self::Swizzle4;

    fn zzzy(self) -> Self::Swizzle4;

    fn zzzz(self) -> Self::Swizzle4;
}

pub trait Swizzles4: Sized + Copy + Clone {
    type Swizzle2;

    type Swizzle3;

    #[inline]
    fn xyzw(self) -> Self {
        self
    }

    fn xx(self) -> Self::Swizzle2;

    fn xy(self) -> Self::Swizzle2;

    fn xz(self) -> Self::Swizzle2;

    fn xw(self) -> Self::Swizzle2;

    fn yx(self) -> Self::Swizzle2;

    fn yy(self) -> Self::Swizzle2;

    fn yz(self) -> Self::Swizzle2;

    fn yw(self) -> Self::Swizzle2;

    fn zx(self) -> Self::Swizzle2;

    fn zy(self) -> Self::Swizzle2;

    fn zz(self) -> Self::Swizzle2;

    fn zw(self) -> Self::Swizzle2;

    fn wx(self) -> Self::Swizzle2;

    fn wy(self) -> Self::Swizzle2;

    fn wz(self) -> Self::Swizzle2;

    fn ww(self) -> Self::Swizzle2;

    fn xxx(self) -> Self::Swizzle3;

    fn xxy(self) -> Self::Swizzle3;

    fn xxz(self) -> Self::Swizzle3;

    fn xxw(self) -> Self::Swizzle3;

    fn xyx(self) -> Self::Swizzle3;

    fn xyy(self) -> Self::Swizzle3;

    fn xyz(self) -> Self::Swizzle3;

    fn xyw(self) -> Self::Swizzle3;

    fn xzx(self) -> Self::Swizzle3;

    fn xzy(self) -> Self::Swizzle3;

    fn xzz(self) -> Self::Swizzle3;

    fn xzw(self) -> Self::Swizzle3;

    fn xwx(self) -> Self::Swizzle3;

    fn xwy(self) -> Self::Swizzle3;

    fn xwz(self) -> Self::Swizzle3;

    fn xww(self) -> Self::Swizzle3;

    fn yxx(self) -> Self::Swizzle3;

    fn yxy(self) -> Self::Swizzle3;

    fn yxz(self) -> Self::Swizzle3;

    fn yxw(self) -> Self::Swizzle3;

    fn yyx(self) -> Self::Swizzle3;

    fn yyy(self) -> Self::Swizzle3;

    fn yyz(self) -> Self::Swizzle3;

    fn yyw(self) -> Self::Swizzle3;

    fn yzx(self) -> Self::Swizzle3;

    fn yzy(self) -> Self::Swizzle3;

    fn yzz(self) -> Self::Swizzle3;

    fn yzw(self) -> Self::Swizzle3;

    fn ywx(self) -> Self::Swizzle3;

    fn ywy(self) -> Self::Swizzle3;

    fn ywz(self) -> Self::Swizzle3;

    fn yww(self) -> Self::Swizzle3;

    fn zxx(self) -> Self::Swizzle3;

    fn zxy(self) -> Self::Swizzle3;

    fn zxz(self) -> Self::Swizzle3;

    fn zxw(self) -> Self::Swizzle3;

    fn zyx(self) -> Self::Swizzle3;

    fn zyy(self) -> Self::Swizzle3;

    fn zyz(self) -> Self::Swizzle3;

    fn zyw(self) -> Self::Swizzle3;

    fn zzx(self) -> Self::Swizzle3;

    fn zzy(self) -> Self::Swizzle3;

    fn zzz(self) -> Self::Swizzle3;

    fn zzw(self) -> Self::Swizzle3;

    fn zwx(self) -> Self::Swizzle3;

    fn zwy(self) -> Self::Swizzle3;

    fn zwz(self) -> Self::Swizzle3;

    fn zww(self) -> Self::Swizzle3;

    fn wxx(self) -> Self::Swizzle3;

    fn wxy(self) -> Self::Swizzle3;

    fn wxz(self) -> Self::Swizzle3;

    fn wxw(self) -> Self::Swizzle3;

    fn wyx(self) -> Self::Swizzle3;

    fn wyy(self) -> Self::Swizzle3;

    fn wyz(self) -> Self::Swizzle3;

    fn wyw(self) -> Self::Swizzle3;

    fn wzx(self) -> Self::Swizzle3;

    fn wzy(self) -> Self::Swizzle3;

    fn wzz(self) -> Self::Swizzle3;

    fn wzw(self) -> Self::Swizzle3;

    fn wwx(self) -> Self::Swizzle3;

    fn wwy(self) -> Self::Swizzle3;

    fn wwz(self) -> Self::Swizzle3;

    fn www(self) -> Self::Swizzle3;

    fn xxxx(self) -> Self;

    fn xxxy(self) -> Self;

    fn xxxz(self) -> Self;

    fn xxxw(self) -> Self;

    fn xxyx(self) -> Self;

    fn xxyy(self) -> Self;

    fn xxyz(self) -> Self;

    fn xxyw(self) -> Self;

    fn xxzx(self) -> Self;

    fn xxzy(self) -> Self;

    fn xxzz(self) -> Self;

    fn xxzw(self) -> Self;

    fn xxwx(self) -> Self;

    fn xxwy(self) -> Self;

    fn xxwz(self) -> Self;

    fn xxww(self) -> Self;

    fn xyxx(self) -> Self;

    fn xyxy(self) -> Self;

    fn xyxz(self) -> Self;

    fn xyxw(self) -> Self;

    fn xyyx(self) -> Self;

    fn xyyy(self) -> Self;

    fn xyyz(self) -> Self;

    fn xyyw(self) -> Self;

    fn xyzx(self) -> Self;

    fn xyzy(self) -> Self;

    fn xyzz(self) -> Self;

    fn xywx(self) -> Self;

    fn xywy(self) -> Self;

    fn xywz(self) -> Self;

    fn xyww(self) -> Self;

    fn xzxx(self) -> Self;

    fn xzxy(self) -> Self;

    fn xzxz(self) -> Self;

    fn xzxw(self) -> Self;

    fn xzyx(self) -> Self;

    fn xzyy(self) -> Self;

    fn xzyz(self) -> Self;

    fn xzyw(self) -> Self;

    fn xzzx(self) -> Self;

    fn xzzy(self) -> Self;

    fn xzzz(self) -> Self;

    fn xzzw(self) -> Self;

    fn xzwx(self) -> Self;

    fn xzwy(self) -> Self;

    fn xzwz(self) -> Self;

    fn xzww(self) -> Self;

    fn xwxx(self) -> Self;

    fn xwxy(self) -> Self;

    fn xwxz(self) -> Self;

    fn xwxw(self) -> Self;

    fn xwyx(self) -> Self;

    fn xwyy(self) -> Self;

    fn xwyz(self) -> Self;

    fn xwyw(self) -> Self;

    fn xwzx(self) -> Self;

    fn xwzy(self) -> Self;

    fn xwzz(self) -> Self;

    fn xwzw(self) -> Self;

    fn xwwx(self) -> Self;

    fn xwwy(self) -> Self;

    fn xwwz(self) -> Self;

    fn xwww(self) -> Self;

    fn yxxx(self) -> Self;

    fn yxxy(self) -> Self;

    fn yxxz(self) -> Self;

    fn yxxw(self) -> Self;

    fn yxyx(self) -> Self;

    fn yxyy(self) -> Self;

    fn yxyz(self) -> Self;

    fn yxyw(self) -> Self;

    fn yxzx(self) -> Self;

    fn yxzy(self) -> Self;

    fn yxzz(self) -> Self;

    fn yxzw(self) -> Self;

    fn yxwx(self) -> Self;

    fn yxwy(self) -> Self;

    fn yxwz(self) -> Self;

    fn yxww(self) -> Self;

    fn yyxx(self) -> Self;

    fn yyxy(self) -> Self;

    fn yyxz(self) -> Self;

    fn yyxw(self) -> Self;

    fn yyyx(self) -> Self;

    fn yyyy(self) -> Self;

    fn yyyz(self) -> Self;

    fn yyyw(self) -> Self;

    fn yyzx(self) -> Self;

    fn yyzy(self) -> Self;

    fn yyzz(self) -> Self;

    fn yyzw(self) -> Self;

    fn yywx(self) -> Self;

    fn yywy(self) -> Self;

    fn yywz(self) -> Self;

    fn yyww(self) -> Self;

    fn yzxx(self) -> Self;

    fn yzxy(self) -> Self;

    fn yzxz(self) -> Self;

    fn yzxw(self) -> Self;

    fn yzyx(self) -> Self;

    fn yzyy(self) -> Self;

    fn yzyz(self) -> Self;

    fn yzyw(self) -> Self;

    fn yzzx(self) -> Self;

    fn yzzy(self) -> Self;

    fn yzzz(self) -> Self;

    fn yzzw(self) -> Self;

    fn yzwx(self) -> Self;

    fn yzwy(self) -> Self;

    fn yzwz(self) -> Self;

    fn yzww(self) -> Self;

    fn ywxx(self) -> Self;

    fn ywxy(self) -> Self;

    fn ywxz(self) -> Self;

    fn ywxw(self) -> Self;

    fn ywyx(self) -> Self;

    fn ywyy(self) -> Self;

    fn ywyz(self) -> Self;

    fn ywyw(self) -> Self;

    fn ywzx(self) -> Self;

    fn ywzy(self) -> Self;

    fn ywzz(self) -> Self;

    fn ywzw(self) -> Self;

    fn ywwx(self) -> Self;

    fn ywwy(self) -> Self;

    fn ywwz(self) -> Self;

    fn ywww(self) -> Self;

    fn zxxx(self) -> Self;

    fn zxxy(self) -> Self;

    fn zxxz(self) -> Self;

    fn zxxw(self) -> Self;

    fn zxyx(self) -> Self;

    fn zxyy(self) -> Self;

    fn zxyz(self) -> Self;

    fn zxyw(self) -> Self;

    fn zxzx(self) -> Self;

    fn zxzy(self) -> Self;

    fn zxzz(self) -> Self;

    fn zxzw(self) -> Self;

    fn zxwx(self) -> Self;

    fn zxwy(self) -> Self;

    fn zxwz(self) -> Self;

    fn zxww(self) -> Self;

    fn zyxx(self) -> Self;

    fn zyxy(self) -> Self;

    fn zyxz(self) -> Self;

    fn zyxw(self) -> Self;

    fn zyyx(self) -> Self;

    fn zyyy(self) -> Self;

    fn zyyz(self) -> Self;

    fn zyyw(self) -> Self;

    fn zyzx(self) -> Self;

    fn zyzy(self) -> Self;

    fn zyzz(self) -> Self;

    fn zyzw(self) -> Self;

    fn zywx(self) -> Self;

    fn zywy(self) -> Self;

    fn zywz(self) -> Self;

    fn zyww(self) -> Self;

    fn zzxx(self) -> Self;

    fn zzxy(self) -> Self;

    fn zzxz(self) -> Self;

    fn zzxw(self) -> Self;

    fn zzyx(self) -> Self;

    fn zzyy(self) -> Self;

    fn zzyz(self) -> Self;

    fn zzyw(self) -> Self;

    fn zzzx(self) -> Self;

    fn zzzy(self) -> Self;

    fn zzzz(self) -> Self;

    fn zzzw(self) -> Self;

    fn zzwx(self) -> Self;

    fn zzwy(self) -> Self;

    fn zzwz(self) -> Self;

    fn zzww(self) -> Self;

    fn zwxx(self) -> Self;

    fn zwxy(self) -> Self;

    fn zwxz(self) -> Self;

    fn zwxw(self) -> Self;

    fn zwyx(self) -> Self;

    fn zwyy(self) -> Self;

    fn zwyz(self) -> Self;

    fn zwyw(self) -> Self;

    fn zwzx(self) -> Self;

    fn zwzy(self) -> Self;

    fn zwzz(self) -> Self;

    fn zwzw(self) -> Self;

    fn zwwx(self) -> Self;

    fn zwwy(self) -> Self;

    fn zwwz(self) -> Self;

    fn zwww(self) -> Self;

    fn wxxx(self) -> Self;

    fn wxxy(self) -> Self;

    fn wxxz(self) -> Self;

    fn wxxw(self) -> Self;

    fn wxyx(self) -> Self;

    fn wxyy(self) -> Self;

    fn wxyz(self) -> Self;

    fn wxyw(self) -> Self;

    fn wxzx(self) -> Self;

    fn wxzy(self) -> Self;

    fn wxzz(self) -> Self;

    fn wxzw(self) -> Self;

    fn wxwx(self) -> Self;

    fn wxwy(self) -> Self;

    fn wxwz(self) -> Self;

    fn wxww(self) -> Self;

    fn wyxx(self) -> Self;

    fn wyxy(self) -> Self;

    fn wyxz(self) -> Self;

    fn wyxw(self) -> Self;

    fn wyyx(self) -> Self;

    fn wyyy(self) -> Self;

    fn wyyz(self) -> Self;

    fn wyyw(self) -> Self;

    fn wyzx(self) -> Self;

    fn wyzy(self) -> Self;

    fn wyzz(self) -> Self;

    fn wyzw(self) -> Self;

    fn wywx(self) -> Self;

    fn wywy(self) -> Self;

    fn wywz(self) -> Self;

    fn wyww(self) -> Self;

    fn wzxx(self) -> Self;

    fn wzxy(self) -> Self;

    fn wzxz(self) -> Self;

    fn wzxw(self) -> Self;

    fn wzyx(self) -> Self;

    fn wzyy(self) -> Self;

    fn wzyz(self) -> Self;

    fn wzyw(self) -> Self;

    fn wzzx(self) -> Self;

    fn wzzy(self) -> Self;

    fn wzzz(self) -> Self;

    fn wzzw(self) -> Self;

    fn wzwx(self) -> Self;

    fn wzwy(self) -> Self;

    fn wzwz(self) -> Self;

    fn wzww(self) -> Self;

    fn wwxx(self) -> Self;

    fn wwxy(self) -> Self;

    fn wwxz(self) -> Self;

    fn wwxw(self) -> Self;

    fn wwyx(self) -> Self;

    fn wwyy(self) -> Self;

    fn wwyz(self) -> Self;

    fn wwyw(self) -> Self;

    fn wwzx(self) -> Self;

    fn wwzy(self) -> Self;

    fn wwzz(self) -> Self;

    fn wwzw(self) -> Self;

    fn wwwx(self) -> Self;

    fn wwwy(self) -> Self;

    fn wwwz(self) -> Self;

    fn wwww(self) -> Self;
}

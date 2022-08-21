00007FF603BD37B0  push         rbp
00007FF603BD37B1  mov          eax, 1660h
00007FF603BD37B6  call         __chkstk (00007FF604CA48A0h)
00007FF603BD37BB  sub          rsp, rax
00007FF603BD37BE  lea          rbp, [rsp+80h]
00007FF603BD37C6  mov          qword ptr [rbp+15D8h], FFFFFFFFFFFFFFFEh
00007FF603BD37D1  movss        dword ptr [rbp+ECh], xmm1
00007FF603BD37D9  mov          qword ptr [rbp+F0h], rcx
00007FF603BD37E0  mov          qword ptr [rbp+F8h], rcx
00007FF603BD37E7  movss        dword ptr [rbp+130Ch], xmm1
00007FF603BD37EF  lea          rax, [rbp+110h]
00007FF603BD37F6  lea          rcx, [rbp+110h]
00007FF603BD37FD  add          rcx, 6C0h
00007FF603BD3804  mov          qword ptr [rbp+100h], rcx
00007FF603BD380B  mov          qword ptr [rbp+108h], rax
00007FF603BD3812  mov          rcx, qword ptr [rbp+100h]
00007FF603BD3819  mov          rax, qword ptr [rbp+108h]
00007FF603BD3820  mov          qword ptr [rbp+E0h], rax
00007FF603BD3827  cmp          rax, rcx
00007FF603BD382A  je           struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+A9h (00007FF603BD3859h)
00007FF603BD382C  mov          rcx, qword ptr [rbp+E0h]
00007FF603BD3833  lea          rdx, [00007FF604D58B60h]
00007FF603BD383A  mov          r8d, 48h
00007FF603BD3840  call         memcpy (00007FF604CA5954h)
00007FF603BD3845  mov          rax, qword ptr [rbp+E0h]
00007FF603BD384C  add          rax, 48h
00007FF603BD3850  mov          qword ptr [rbp+108h], rax
00007FF603BD3857  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+62h (00007FF603BD3812h)
00007FF603BD3859  lea          rcx, [rbp+7D0h]
00007FF603BD3860  xor          edx, edx
00007FF603BD3862  mov          r8d, 90h
00007FF603BD3868  call         memset (00007FF604CA595Ah)
00007FF603BD386D  mov          qword ptr [rbp+860h], 0h
00007FF603BD3878  mov          qword ptr [rbp+868h], 6h
00007FF603BD3883  mov          rcx, qword ptr [rbp+860h]
00007FF603BD388A  mov          rdx, qword ptr [rbp+868h]
00007FF603BD3891  call         struct core::ops::range::Range<usize> core::iter::traits::collect::impl$0::into_iter<core::ops::range::Range<usize> > (00007FF6048080C0h)
00007FF603BD3896  mov          qword ptr [rbp+870h], rax
00007FF603BD389D  mov          qword ptr [rbp+878h], rdx
00007FF603BD38A4  lea          rcx, [rbp+870h]
00007FF603BD38AB  call         union enum$<core::option::Option<usize> > core::iter::range::impl$3::next<usize> (00007FF604807FE0h)
00007FF603BD38B0  mov          qword ptr [rbp+888h], rdx
00007FF603BD38B7  mov          qword ptr [rbp+880h], rax
00007FF603BD38BE  mov          rax, qword ptr [rbp+880h]
00007FF603BD38C5  test         rax, rax
00007FF603BD38C8  je           struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+120h (00007FF603BD38D0h)
00007FF603BD38CA  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+11Ch (00007FF603BD38CCh)
00007FF603BD38CC  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+157h (00007FF603BD3907h)
00007FF603BD38CE  ud2
00007FF603BD38D0  lea          rcx, [rbp+10D8h]
00007FF603BD38D7  lea          rdx, [rbp+110h]
00007FF603BD38DE  mov          r8d, 18h
00007FF603BD38E4  call         struct alloc::vec::Vec<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> alloc::slice::impl$0::to_vec<fabled_render::mesh::container::vertex::Vertex> (00007FF603BDA3C0h)
00007FF603BD38E9  lea          rcx, [rbp+1200h]
00007FF603BD38F0  lea          rdx, [rbp+7D0h]
00007FF603BD38F7  mov          r8d, 24h
00007FF603BD38FD  call         struct smallvec::SmallVec<array$<u32,64> > smallvec::impl$61::to_smallvec<array$<u32,64> > (00007FF603BD46E0h)
00007FF603BD3902  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+DCFh (00007FF603BD457Fh)
00007FF603BD3907  mov          rax, qword ptr [rbp+888h]
00007FF603BD390E  mov          qword ptr [rbp+D8h], rax
00007FF603BD3915  mov          qword ptr [rbp+1310h], rax
00007FF603BD391C  mov          dword ptr [rbp+D4h], eax
00007FF603BD3922  mov          dword ptr [rbp+131Ch], eax
00007FF603BD3928  lea          rcx, [rbp+890h]
00007FF603BD392F  lea          rdx, [00007FF604D58BB0h]
00007FF603BD3936  mov          r8d, 120h
00007FF603BD393C  call         memcpy (00007FF604CA5954h)
00007FF603BD3941  mov          rax, qword ptr [rbp+D8h]
00007FF603BD3948  cmp          rax, 6h
00007FF603BD394C  setb         al
00007FF603BD394F  test         al, 1h
00007FF603BD3951  jne          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+1A5h (00007FF603BD3955h)
00007FF603BD3953  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+1F8h (00007FF603BD39A8h)
00007FF603BD3955  mov          rcx, qword ptr [rbp+D8h]
00007FF603BD395C  lea          rax, [rbp+890h]
00007FF603BD3963  shl          rcx, 4h
00007FF603BD3967  add          rax, rcx
00007FF603BD396A  movaps       xmm0, xmmword ptr [rax]
00007FF603BD396D  movaps       xmmword ptr [rbp+C0h], xmm0
00007FF603BD3974  movaps       xmmword ptr [rbp+1320h], xmm0
00007FF603BD397B  lea          rcx, [rbp+9B0h]
00007FF603BD3982  lea          rdx, [00007FF604D58BB0h]
00007FF603BD3989  mov          r8d, 120h
00007FF603BD398F  call         memcpy (00007FF604CA5954h)
00007FF603BD3994  mov          rax, qword ptr [rbp+D8h]
00007FF603BD399B  cmp          rax, 6h
00007FF603BD399F  setb         al
00007FF603BD39A2  test         al, 1h
00007FF603BD39A4  jne          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+212h (00007FF603BD39C2h)
00007FF603BD39A6  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+269h (00007FF603BD3A19h)
00007FF603BD39A8  mov          rcx, qword ptr [rbp+D8h]
00007FF603BD39AF  lea          r8, [00007FF604D58D08h]
00007FF603BD39B6  mov          edx, 6h
00007FF603BD39BB  call         void core::panicking::panic_bounds_check (00007FF604CA71A0h)
00007FF603BD39C0  ud2
00007FF603BD39C2  mov          rcx, qword ptr [rbp+D8h]
00007FF603BD39C9  lea          rax, [rbp+9B0h]
00007FF603BD39D0  add          rax, 60h
00007FF603BD39D4  shl          rcx, 4h
00007FF603BD39D8  add          rax, rcx
00007FF603BD39DB  movaps       xmm0, xmmword ptr [rax]
00007FF603BD39DE  movaps       xmmword ptr [rbp+B0h], xmm0
00007FF603BD39E5  movaps       xmmword ptr [rbp+1330h], xmm0
00007FF603BD39EC  lea          rcx, [rbp+AD0h]
00007FF603BD39F3  lea          rdx, [00007FF604D58BB0h]
00007FF603BD39FA  mov          r8d, 120h
00007FF603BD3A00  call         memcpy (00007FF604CA5954h)
00007FF603BD3A05  mov          rax, qword ptr [rbp+D8h]
00007FF603BD3A0C  cmp          rax, 6h
00007FF603BD3A10  setb         al
00007FF603BD3A13  test         al, 1h
00007FF603BD3A15  jne          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+283h (00007FF603BD3A33h)
00007FF603BD3A17  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+2D7h (00007FF603BD3A87h)
00007FF603BD3A19  mov          rcx, qword ptr [rbp+D8h]
00007FF603BD3A20  lea          r8, [00007FF604D58D20h]
00007FF603BD3A27  mov          edx, 6h
00007FF603BD3A2C  call         void core::panicking::panic_bounds_check (00007FF604CA71A0h)
00007FF603BD3A31  ud2
00007FF603BD3A33  mov          eax, dword ptr [rbp+D4h]
00007FF603BD3A39  mov          rdx, qword ptr [rbp+D8h]
00007FF603BD3A40  lea          rcx, [rbp+AD0h]
00007FF603BD3A47  add          rcx, C0h
00007FF603BD3A4E  shl          rdx, 4h
00007FF603BD3A52  add          rcx, rdx
00007FF603BD3A55  movaps       xmm0, xmmword ptr [rcx]
00007FF603BD3A58  movaps       xmmword ptr [rbp+90h], xmm0
00007FF603BD3A5F  movaps       xmmword ptr [rbp+1340h], xmm0
00007FF603BD3A66  shl          eax, 2h
00007FF603BD3A69  mov          dword ptr [rbp+A8h], eax
00007FF603BD3A6F  mov          dword ptr [rbp+135Ch], eax
00007FF603BD3A75  add          eax, 2h
00007FF603BD3A78  mov          dword ptr [rbp+ACh], eax
00007FF603BD3A7E  setb         al
00007FF603BD3A81  test         al, 1h
00007FF603BD3A83  jne          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+309h (00007FF603BD3AB9h)
00007FF603BD3A85  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+2F1h (00007FF603BD3AA1h)
00007FF603BD3A87  mov          rcx, qword ptr [rbp+D8h]
00007FF603BD3A8E  lea          r8, [00007FF604D58D38h]
00007FF603BD3A95  mov          edx, 6h
00007FF603BD3A9A  call         void core::panicking::panic_bounds_check (00007FF604CA71A0h)
00007FF603BD3A9F  ud2
00007FF603BD3AA1  mov          eax, dword ptr [rbp+A8h]
00007FF603BD3AA7  add          eax, 1h
00007FF603BD3AAA  mov          dword ptr [rbp+8Ch], eax
00007FF603BD3AB0  setb         al
00007FF603BD3AB3  test         al, 1h
00007FF603BD3AB5  jne          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+33Bh (00007FF603BD3AEBh)
00007FF603BD3AB7  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+323h (00007FF603BD3AD3h)
00007FF603BD3AB9  lea          rcx, [00007FF604D58D70h]
00007FF603BD3AC0  lea          r8, [00007FF604D58D50h]
00007FF603BD3AC7  mov          edx, 1Ch
00007FF603BD3ACC  call         void core::panicking::panic (00007FF604CA7150h)
00007FF603BD3AD1  ud2
00007FF603BD3AD3  mov          eax, dword ptr [rbp+A8h]
00007FF603BD3AD9  add          eax, 3h
00007FF603BD3ADC  mov          dword ptr [rbp+88h], eax
00007FF603BD3AE2  setb         al
00007FF603BD3AE5  test         al, 1h
00007FF603BD3AE7  jne          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+371h (00007FF603BD3B21h)
00007FF603BD3AE9  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+355h (00007FF603BD3B05h)
00007FF603BD3AEB  lea          rcx, [00007FF604D58D70h]
00007FF603BD3AF2  lea          r8, [00007FF604D58D90h]
00007FF603BD3AF9  mov          edx, 1Ch
00007FF603BD3AFE  call         void core::panicking::panic (00007FF604CA7150h)
00007FF603BD3B03  ud2
00007FF603BD3B05  mov          eax, dword ptr [rbp+A8h]
00007FF603BD3B0B  add          eax, 2h
00007FF603BD3B0E  mov          dword ptr [rbp+84h], eax
00007FF603BD3B14  setb         al
00007FF603BD3B17  test         al, 1h
00007FF603BD3B19  jne          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+3F4h (00007FF603BD3BA4h)
00007FF603BD3B1F  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+38Bh (00007FF603BD3B3Bh)
00007FF603BD3B21  lea          rcx, [00007FF604D58D70h]
00007FF603BD3B28  lea          r8, [00007FF604D58DA8h]
00007FF603BD3B2F  mov          edx, 1Ch
00007FF603BD3B34  call         void core::panicking::panic (00007FF604CA7150h)
00007FF603BD3B39  ud2
00007FF603BD3B3B  mov          rax, qword ptr [rbp+D8h]
00007FF603BD3B42  mov          ecx, dword ptr [rbp+84h]
00007FF603BD3B48  mov          edx, dword ptr [rbp+88h]
00007FF603BD3B4E  mov          r8d, dword ptr [rbp+A8h]
00007FF603BD3B55  mov          r9d, dword ptr [rbp+8Ch]
00007FF603BD3B5C  mov          r10d, dword ptr [rbp+ACh]
00007FF603BD3B63  mov          dword ptr [rbp+BF0h], r10d
00007FF603BD3B6A  mov          dword ptr [rbp+BF4h], r9d
00007FF603BD3B71  mov          dword ptr [rbp+BF8h], r8d
00007FF603BD3B78  mov          dword ptr [rbp+BFCh], r8d
00007FF603BD3B7F  mov          dword ptr [rbp+C00h], edx
00007FF603BD3B85  mov          dword ptr [rbp+C04h], ecx
00007FF603BD3B8B  mov          ecx, 6h
00007FF603BD3B90  mul          rcx
00007FF603BD3B93  mov          qword ptr [rbp+78h], rax
00007FF603BD3B97  seto         al
00007FF603BD3B9A  test         al, 1h
00007FF603BD3B9C  jne          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+5B1h (00007FF603BD3D61h)
00007FF603BD3BA2  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+40Eh (00007FF603BD3BBEh)
00007FF603BD3BA4  lea          rcx, [00007FF604D58D70h]
00007FF603BD3BAB  lea          r8, [00007FF604D58DC0h]
00007FF603BD3BB2  mov          edx, 1Ch
00007FF603BD3BB7  call         void core::panicking::panic (00007FF604CA7150h)
00007FF603BD3BBC  ud2
00007FF603BD3BBE  mov          rax, qword ptr [rbp+78h]
00007FF603BD3BC2  mov          qword ptr [rbp+1360h], rax
00007FF603BD3BC9  mov          qword ptr [rbp+C20h], rax
00007FF603BD3BD0  mov          rdx, qword ptr [rbp+C20h]
00007FF603BD3BD7  lea          rcx, [rbp+7D0h]
00007FF603BD3BDE  lea          r8, [00007FF604D58E18h]
00007FF603BD3BE5  call         struct slice$<u32> core::array::impl$16::index_mut<u32,core::ops::range::RangeFrom<usize>,36> (00007FF603BDA520h)
00007FF603BD3BEA  mov          qword ptr [rbp+60h], rax
00007FF603BD3BEE  mov          r8, rdx
00007FF603BD3BF1  mov          rdx, qword ptr [rbp+60h]
00007FF603BD3BF5  lea          rcx, [rbp+C08h]
00007FF603BD3BFC  lea          r9, [00007FF604D58E30h]
00007FF603BD3C03  call         struct tuple$<ref_mut$<array$<u32,6> >,slice$<u32> > core::slice::impl$0::split_array_mut<u32,6> (00007FF603BDA870h)
00007FF603BD3C08  mov          rcx, qword ptr [rbp+C08h]
00007FF603BD3C0F  mov          qword ptr [rbp+1368h], rcx
00007FF603BD3C16  lea          r8, [rbp+BF0h]
00007FF603BD3C1D  mov          r9d, 6h
00007FF603BD3C23  lea          rax, [00007FF604D58E48h]
00007FF603BD3C2A  mov          rdx, r9
00007FF603BD3C2D  mov          qword ptr [rsp+20h], rax
00007FF603BD3C32  call         void core::slice::impl$0::copy_from_slice<u32> (00007FF603BDA7B0h)
00007FF603BD3C37  movaps       xmm0, xmmword ptr [rbp+C0h]
00007FF603BD3C3E  movaps       xmmword ptr [rbp+1370h], xmm0
00007FF603BD3C45  lea          rcx, [rbp+C28h]
00007FF603BD3C4C  lea          rdx, [rbp+1370h]
00007FF603BD3C53  call         float[4] fabled_math::linear::vec3::Vector3::to_primitive (00007FF603BDAC50h)
00007FF603BD3C58  movaps       xmm1, xmmword ptr [rbp+C0h]
00007FF603BD3C5F  movaps       xmm0, xmmword ptr [rbp+90h]
00007FF603BD3C66  movaps       xmmword ptr [rbp+1390h], xmm1
00007FF603BD3C6D  movaps       xmmword ptr [rbp+13A0h], xmm0
00007FF603BD3C74  lea          rcx, [rbp+1380h]
00007FF603BD3C7B  lea          rdx, [rbp+1390h]
00007FF603BD3C82  lea          r8, [rbp+13A0h]
00007FF603BD3C89  call         struct fabled_math::linear::vec3::Vector3 fabled_math::linear::vec3::impl$27::sub (00007FF603BDACE0h)
00007FF603BD3C8E  movaps       xmm0, xmmword ptr [rbp+B0h]
00007FF603BD3C95  movaps       xmm1, xmmword ptr [rbp+1380h]
00007FF603BD3C9C  movaps       xmmword ptr [rbp+13C0h], xmm1
00007FF603BD3CA3  movaps       xmmword ptr [rbp+13D0h], xmm0
00007FF603BD3CAA  lea          rcx, [rbp+13B0h]
00007FF603BD3CB1  lea          rdx, [rbp+13C0h]
00007FF603BD3CB8  lea          r8, [rbp+13D0h]
00007FF603BD3CBF  call         struct fabled_math::linear::vec3::Vector3 fabled_math::linear::vec3::impl$27::sub (00007FF603BDACE0h)
00007FF603BD3CC4  movss        xmm2, dword ptr [rbp+ECh]
00007FF603BD3CCC  movaps       xmm0, xmmword ptr [rbp+13B0h]
00007FF603BD3CD3  movaps       xmmword ptr [rbp+13F0h], xmm0
00007FF603BD3CDA  lea          rcx, [rbp+13E0h]
00007FF603BD3CE1  lea          rdx, [rbp+13F0h]
00007FF603BD3CE8  call         struct fabled_math::linear::vec3::Vector3 fabled_math::linear::vec3::impl$18::mul (00007FF603BDAD40h)
00007FF603BD3CED  movaps       xmm0, xmmword ptr [rbp+13E0h]
00007FF603BD3CF4  movaps       xmmword ptr [rbp+1400h], xmm0
00007FF603BD3CFB  lea          rcx, [rbp+C80h]
00007FF603BD3D02  lea          rdx, [rbp+1400h]
00007FF603BD3D09  call         float[4] fabled_math::linear::vec3::Vector3::to_primitive (00007FF603BDAC50h)
00007FF603BD3D0E  movss        xmm0, dword ptr [__real@3f800000 (00007FF604CBE4C4h)]
00007FF603BD3D16  movss        dword ptr [rbp+C90h], xmm0
00007FF603BD3D1E  xorps        xmm0, xmm0
00007FF603BD3D21  movss        dword ptr [rbp+C94h], xmm0
00007FF603BD3D29  mov          rax, qword ptr [rbp+C28h]
00007FF603BD3D30  mov          qword ptr [rbp+C98h], rax
00007FF603BD3D37  mov          rax, qword ptr [rbp+C30h]
00007FF603BD3D3E  mov          qword ptr [rbp+CA0h], rax
00007FF603BD3D45  lea          rax, [rbp+CA8h]
00007FF603BD3D4C  lea          rcx, [rbp+CA8h]
00007FF603BD3D53  add          rcx, 10h
00007FF603BD3D57  mov          qword ptr [rbp+68h], rcx
00007FF603BD3D5B  mov          qword ptr [rbp+70h], rax
00007FF603BD3D5F  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+5CBh (00007FF603BD3D7Bh)
00007FF603BD3D61  lea          rcx, [00007FF604D58DF0h]
00007FF603BD3D68  lea          r8, [00007FF604D58DD8h]
00007FF603BD3D6F  mov          edx, 21h
00007FF603BD3D74  call         void core::panicking::panic (00007FF604CA7150h)
00007FF603BD3D79  ud2
00007FF603BD3D7B  mov          rcx, qword ptr [rbp+68h]
00007FF603BD3D7F  mov          rax, qword ptr [rbp+70h]
00007FF603BD3D83  mov          qword ptr [rbp+58h], rax
00007FF603BD3D87  cmp          rax, rcx
00007FF603BD3D8A  je           struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+5F1h (00007FF603BD3DA1h)
00007FF603BD3D8C  mov          rax, qword ptr [rbp+58h]
00007FF603BD3D90  xorps        xmm0, xmm0
00007FF603BD3D93  movss        dword ptr [rax], xmm0
00007FF603BD3D97  add          rax, 4h
00007FF603BD3D9B  mov          qword ptr [rbp+70h], rax
00007FF603BD3D9F  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+5CBh (00007FF603BD3D7Bh)
00007FF603BD3DA1  lea          rax, [rbp+CB8h]
00007FF603BD3DA8  lea          rcx, [rbp+CB8h]
00007FF603BD3DAF  add          rcx, 10h
00007FF603BD3DB3  mov          qword ptr [rbp+48h], rcx
00007FF603BD3DB7  mov          qword ptr [rbp+50h], rax
00007FF603BD3DBB  mov          rcx, qword ptr [rbp+48h]
00007FF603BD3DBF  mov          rax, qword ptr [rbp+50h]
00007FF603BD3DC3  mov          qword ptr [rbp+40h], rax
00007FF603BD3DC7  cmp          rax, rcx
00007FF603BD3DCA  je           struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+631h (00007FF603BD3DE1h)
00007FF603BD3DCC  mov          rax, qword ptr [rbp+40h]
00007FF603BD3DD0  xorps        xmm0, xmm0
00007FF603BD3DD3  movss        dword ptr [rax], xmm0
00007FF603BD3DD7  add          rax, 4h
00007FF603BD3DDB  mov          qword ptr [rbp+50h], rax
00007FF603BD3DDF  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+60Bh (00007FF603BD3DBBh)
00007FF603BD3DE1  movaps       xmm0, xmmword ptr [rbp+90h]
00007FF603BD3DE8  movaps       xmm1, xmmword ptr [rbp+C0h]
00007FF603BD3DEF  mov          rax, qword ptr [rbp+CA8h]
00007FF603BD3DF6  mov          qword ptr [rbp+C38h], rax
00007FF603BD3DFD  mov          rax, qword ptr [rbp+CB0h]
00007FF603BD3E04  mov          qword ptr [rbp+C40h], rax
00007FF603BD3E0B  mov          rax, qword ptr [rbp+CB8h]
00007FF603BD3E12  mov          qword ptr [rbp+C48h], rax
00007FF603BD3E19  mov          rax, qword ptr [rbp+CC0h]
00007FF603BD3E20  mov          qword ptr [rbp+C50h], rax
00007FF603BD3E27  mov          rax, qword ptr [rbp+C80h]
00007FF603BD3E2E  mov          qword ptr [rbp+C58h], rax
00007FF603BD3E35  mov          rax, qword ptr [rbp+C88h]
00007FF603BD3E3C  mov          qword ptr [rbp+C60h], rax
00007FF603BD3E43  mov          rax, qword ptr [rbp+C98h]
00007FF603BD3E4A  mov          qword ptr [rbp+C68h], rax
00007FF603BD3E51  mov          rax, qword ptr [rbp+CA0h]
00007FF603BD3E58  mov          qword ptr [rbp+C70h], rax
00007FF603BD3E5F  mov          rax, qword ptr [rbp+C90h]
00007FF603BD3E66  mov          qword ptr [rbp+C78h], rax
00007FF603BD3E6D  movaps       xmmword ptr [rbp+1420h], xmm1
00007FF603BD3E74  movaps       xmmword ptr [rbp+1430h], xmm0
00007FF603BD3E7B  lea          rcx, [rbp+1410h]
00007FF603BD3E82  lea          rdx, [rbp+1420h]
00007FF603BD3E89  lea          r8, [rbp+1430h]
00007FF603BD3E90  call         struct fabled_math::linear::vec3::Vector3 fabled_math::linear::vec3::impl$27::sub (00007FF603BDACE0h)
00007FF603BD3E95  movaps       xmm0, xmmword ptr [rbp+B0h]
00007FF603BD3E9C  movaps       xmm1, xmmword ptr [rbp+1410h]
00007FF603BD3EA3  movaps       xmmword ptr [rbp+1450h], xmm1
00007FF603BD3EAA  movaps       xmmword ptr [rbp+1460h], xmm0
00007FF603BD3EB1  lea          rcx, [rbp+1440h]
00007FF603BD3EB8  lea          rdx, [rbp+1450h]
00007FF603BD3EBF  lea          r8, [rbp+1460h]
00007FF603BD3EC6  call         struct fabled_math::linear::vec3::Vector3 fabled_math::linear::vec3::impl$25::add (00007FF603BDAC80h)
00007FF603BD3ECB  movss        xmm2, dword ptr [rbp+ECh]
00007FF603BD3ED3  movaps       xmm0, xmmword ptr [rbp+1440h]
00007FF603BD3EDA  movaps       xmmword ptr [rbp+1480h], xmm0
00007FF603BD3EE1  lea          rcx, [rbp+1470h]
00007FF603BD3EE8  lea          rdx, [rbp+1480h]
00007FF603BD3EEF  call         struct fabled_math::linear::vec3::Vector3 fabled_math::linear::vec3::impl$18::mul (00007FF603BDAD40h)
00007FF603BD3EF4  movaps       xmm0, xmmword ptr [rbp+1470h]
00007FF603BD3EFB  movaps       xmmword ptr [rbp+1490h], xmm0
00007FF603BD3F02  lea          rcx, [rbp+D10h]
00007FF603BD3F09  lea          rdx, [rbp+1490h]
00007FF603BD3F10  call         float[4] fabled_math::linear::vec3::Vector3::to_primitive (00007FF603BDAC50h)
00007FF603BD3F15  movss        xmm0, dword ptr [__real@3f800000 (00007FF604CBE4C4h)]
00007FF603BD3F1D  movss        dword ptr [rbp+D20h], xmm0
00007FF603BD3F25  movss        xmm0, dword ptr [__real@3f800000 (00007FF604CBE4C4h)]
00007FF603BD3F2D  movss        dword ptr [rbp+D24h], xmm0
00007FF603BD3F35  mov          rax, qword ptr [rbp+C28h]
00007FF603BD3F3C  mov          qword ptr [rbp+D28h], rax
00007FF603BD3F43  mov          rax, qword ptr [rbp+C30h]
00007FF603BD3F4A  mov          qword ptr [rbp+D30h], rax
00007FF603BD3F51  lea          rax, [rbp+D38h]
00007FF603BD3F58  lea          rcx, [rbp+D38h]
00007FF603BD3F5F  add          rcx, 10h
00007FF603BD3F63  mov          qword ptr [rbp+30h], rcx
00007FF603BD3F67  mov          qword ptr [rbp+38h], rax
00007FF603BD3F6B  mov          rcx, qword ptr [rbp+30h]
00007FF603BD3F6F  mov          rax, qword ptr [rbp+38h]
00007FF603BD3F73  mov          qword ptr [rbp+28h], rax
00007FF603BD3F77  cmp          rax, rcx
00007FF603BD3F7A  je           struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+7E1h (00007FF603BD3F91h)
00007FF603BD3F7C  mov          rax, qword ptr [rbp+28h]
00007FF603BD3F80  xorps        xmm0, xmm0
00007FF603BD3F83  movss        dword ptr [rax], xmm0
00007FF603BD3F87  add          rax, 4h
00007FF603BD3F8B  mov          qword ptr [rbp+38h], rax
00007FF603BD3F8F  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+7BBh (00007FF603BD3F6Bh)
00007FF603BD3F91  lea          rax, [rbp+D48h]
00007FF603BD3F98  lea          rcx, [rbp+D48h]
00007FF603BD3F9F  add          rcx, 10h
00007FF603BD3FA3  mov          qword ptr [rbp+18h], rcx
00007FF603BD3FA7  mov          qword ptr [rbp+20h], rax
00007FF603BD3FAB  mov          rcx, qword ptr [rbp+18h]
00007FF603BD3FAF  mov          rax, qword ptr [rbp+20h]
00007FF603BD3FB3  mov          qword ptr [rbp+10h], rax
00007FF603BD3FB7  cmp          rax, rcx
00007FF603BD3FBA  je           struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+821h (00007FF603BD3FD1h)
00007FF603BD3FBC  mov          rax, qword ptr [rbp+10h]
00007FF603BD3FC0  xorps        xmm0, xmm0
00007FF603BD3FC3  movss        dword ptr [rax], xmm0
00007FF603BD3FC7  add          rax, 4h
00007FF603BD3FCB  mov          qword ptr [rbp+20h], rax
00007FF603BD3FCF  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+7FBh (00007FF603BD3FABh)
00007FF603BD3FD1  movaps       xmm0, xmmword ptr [rbp+90h]
00007FF603BD3FD8  movaps       xmm1, xmmword ptr [rbp+C0h]
00007FF603BD3FDF  mov          rax, qword ptr [rbp+D38h]
00007FF603BD3FE6  mov          qword ptr [rbp+CC8h], rax
00007FF603BD3FED  mov          rax, qword ptr [rbp+D40h]
00007FF603BD3FF4  mov          qword ptr [rbp+CD0h], rax
00007FF603BD3FFB  mov          rax, qword ptr [rbp+D48h]
00007FF603BD4002  mov          qword ptr [rbp+CD8h], rax
00007FF603BD4009  mov          rax, qword ptr [rbp+D50h]
00007FF603BD4010  mov          qword ptr [rbp+CE0h], rax
00007FF603BD4017  mov          rax, qword ptr [rbp+D10h]
00007FF603BD401E  mov          qword ptr [rbp+CE8h], rax
00007FF603BD4025  mov          rax, qword ptr [rbp+D18h]
00007FF603BD402C  mov          qword ptr [rbp+CF0h], rax
00007FF603BD4033  mov          rax, qword ptr [rbp+D28h]
00007FF603BD403A  mov          qword ptr [rbp+CF8h], rax
00007FF603BD4041  mov          rax, qword ptr [rbp+D30h]
00007FF603BD4048  mov          qword ptr [rbp+D00h], rax
00007FF603BD404F  mov          rax, qword ptr [rbp+D20h]
00007FF603BD4056  mov          qword ptr [rbp+D08h], rax
00007FF603BD405D  movaps       xmmword ptr [rbp+14B0h], xmm1
00007FF603BD4064  movaps       xmmword ptr [rbp+14C0h], xmm0
00007FF603BD406B  lea          rcx, [rbp+14A0h]
00007FF603BD4072  lea          rdx, [rbp+14B0h]
00007FF603BD4079  lea          r8, [rbp+14C0h]
00007FF603BD4080  call         struct fabled_math::linear::vec3::Vector3 fabled_math::linear::vec3::impl$25::add (00007FF603BDAC80h)
00007FF603BD4085  movaps       xmm0, xmmword ptr [rbp+B0h]
00007FF603BD408C  movaps       xmm1, xmmword ptr [rbp+14A0h]
00007FF603BD4093  movaps       xmmword ptr [rbp+14E0h], xmm1
00007FF603BD409A  movaps       xmmword ptr [rbp+14F0h], xmm0
00007FF603BD40A1  lea          rcx, [rbp+14D0h]
00007FF603BD40A8  lea          rdx, [rbp+14E0h]
00007FF603BD40AF  lea          r8, [rbp+14F0h]
00007FF603BD40B6  call         struct fabled_math::linear::vec3::Vector3 fabled_math::linear::vec3::impl$25::add (00007FF603BDAC80h)
00007FF603BD40BB  movss        xmm2, dword ptr [rbp+ECh]
00007FF603BD40C3  movaps       xmm0, xmmword ptr [rbp+14D0h]
00007FF603BD40CA  movaps       xmmword ptr [rbp+1510h], xmm0
00007FF603BD40D1  lea          rcx, [rbp+1500h]
00007FF603BD40D8  lea          rdx, [rbp+1510h]
00007FF603BD40DF  call         struct fabled_math::linear::vec3::Vector3 fabled_math::linear::vec3::impl$18::mul (00007FF603BDAD40h)
00007FF603BD40E4  movaps       xmm0, xmmword ptr [rbp+1500h]
00007FF603BD40EB  movaps       xmmword ptr [rbp+1520h], xmm0
00007FF603BD40F2  lea          rcx, [rbp+DA0h]
00007FF603BD40F9  lea          rdx, [rbp+1520h]
00007FF603BD4100  call         float[4] fabled_math::linear::vec3::Vector3::to_primitive (00007FF603BDAC50h)
00007FF603BD4105  xorps        xmm0, xmm0
00007FF603BD4108  movss        dword ptr [rbp+DB0h], xmm0
00007FF603BD4110  movss        xmm0, dword ptr [__real@3f800000 (00007FF604CBE4C4h)]
00007FF603BD4118  movss        dword ptr [rbp+DB4h], xmm0
00007FF603BD4120  mov          rax, qword ptr [rbp+C28h]
00007FF603BD4127  mov          qword ptr [rbp+DB8h], rax
00007FF603BD412E  mov          rax, qword ptr [rbp+C30h]
00007FF603BD4135  mov          qword ptr [rbp+DC0h], rax
00007FF603BD413C  lea          rax, [rbp+DC8h]
00007FF603BD4143  lea          rcx, [rbp+DC8h]
00007FF603BD414A  add          rcx, 10h
00007FF603BD414E  mov          qword ptr [rbp], rcx
00007FF603BD4152  mov          qword ptr [rbp+8h], rax
00007FF603BD4156  mov          rcx, qword ptr [rbp]
00007FF603BD415A  mov          rax, qword ptr [rbp+8h]
00007FF603BD415E  mov          qword ptr [rbp-8h], rax
00007FF603BD4162  cmp          rax, rcx
00007FF603BD4165  je           struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+9CCh (00007FF603BD417Ch)
00007FF603BD4167  mov          rax, qword ptr [rbp-8h]
00007FF603BD416B  xorps        xmm0, xmm0
00007FF603BD416E  movss        dword ptr [rax], xmm0
00007FF603BD4172  add          rax, 4h
00007FF603BD4176  mov          qword ptr [rbp+8h], rax
00007FF603BD417A  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+9A6h (00007FF603BD4156h)
00007FF603BD417C  lea          rax, [rbp+DD8h]
00007FF603BD4183  lea          rcx, [rbp+DD8h]
00007FF603BD418A  add          rcx, 10h
00007FF603BD418E  mov          qword ptr [rbp-18h], rcx
00007FF603BD4192  mov          qword ptr [rbp-10h], rax
00007FF603BD4196  mov          rcx, qword ptr [rbp-18h]
00007FF603BD419A  mov          rax, qword ptr [rbp-10h]
00007FF603BD419E  mov          qword ptr [rbp-20h], rax
00007FF603BD41A2  cmp          rax, rcx
00007FF603BD41A5  je           struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+A0Ch (00007FF603BD41BCh)
00007FF603BD41A7  mov          rax, qword ptr [rbp-20h]
00007FF603BD41AB  xorps        xmm0, xmm0
00007FF603BD41AE  movss        dword ptr [rax], xmm0
00007FF603BD41B2  add          rax, 4h
00007FF603BD41B6  mov          qword ptr [rbp-10h], rax
00007FF603BD41BA  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+9E6h (00007FF603BD4196h)
00007FF603BD41BC  movaps       xmm0, xmmword ptr [rbp+90h]
00007FF603BD41C3  movaps       xmm1, xmmword ptr [rbp+C0h]
00007FF603BD41CA  mov          rax, qword ptr [rbp+DC8h]
00007FF603BD41D1  mov          qword ptr [rbp+D58h], rax
00007FF603BD41D8  mov          rax, qword ptr [rbp+DD0h]
00007FF603BD41DF  mov          qword ptr [rbp+D60h], rax
00007FF603BD41E6  mov          rax, qword ptr [rbp+DD8h]
00007FF603BD41ED  mov          qword ptr [rbp+D68h], rax
00007FF603BD41F4  mov          rax, qword ptr [rbp+DE0h]
00007FF603BD41FB  mov          qword ptr [rbp+D70h], rax
00007FF603BD4202  mov          rax, qword ptr [rbp+DA0h]
00007FF603BD4209  mov          qword ptr [rbp+D78h], rax
00007FF603BD4210  mov          rax, qword ptr [rbp+DA8h]
00007FF603BD4217  mov          qword ptr [rbp+D80h], rax
00007FF603BD421E  mov          rax, qword ptr [rbp+DB8h]
00007FF603BD4225  mov          qword ptr [rbp+D88h], rax
00007FF603BD422C  mov          rax, qword ptr [rbp+DC0h]
00007FF603BD4233  mov          qword ptr [rbp+D90h], rax
00007FF603BD423A  mov          rax, qword ptr [rbp+DB0h]
00007FF603BD4241  mov          qword ptr [rbp+D98h], rax
00007FF603BD4248  movaps       xmmword ptr [rbp+1540h], xmm1
00007FF603BD424F  movaps       xmmword ptr [rbp+1550h], xmm0
00007FF603BD4256  lea          rcx, [rbp+1530h]
00007FF603BD425D  lea          rdx, [rbp+1540h]
00007FF603BD4264  lea          r8, [rbp+1550h]
00007FF603BD426B  call         struct fabled_math::linear::vec3::Vector3 fabled_math::linear::vec3::impl$25::add (00007FF603BDAC80h)
00007FF603BD4270  movaps       xmm0, xmmword ptr [rbp+B0h]
00007FF603BD4277  movaps       xmm1, xmmword ptr [rbp+1530h]
00007FF603BD427E  movaps       xmmword ptr [rbp+1570h], xmm1
00007FF603BD4285  movaps       xmmword ptr [rbp+1580h], xmm0
00007FF603BD428C  lea          rcx, [rbp+1560h]
00007FF603BD4293  lea          rdx, [rbp+1570h]
00007FF603BD429A  lea          r8, [rbp+1580h]
00007FF603BD42A1  call         struct fabled_math::linear::vec3::Vector3 fabled_math::linear::vec3::impl$27::sub (00007FF603BDACE0h)
00007FF603BD42A6  movss        xmm2, dword ptr [rbp+ECh]
00007FF603BD42AE  movaps       xmm0, xmmword ptr [rbp+1560h]
00007FF603BD42B5  movaps       xmmword ptr [rbp+15A0h], xmm0
00007FF603BD42BC  lea          rcx, [rbp+1590h]
00007FF603BD42C3  lea          rdx, [rbp+15A0h]
00007FF603BD42CA  call         struct fabled_math::linear::vec3::Vector3 fabled_math::linear::vec3::impl$18::mul (00007FF603BDAD40h)
00007FF603BD42CF  movaps       xmm0, xmmword ptr [rbp+1590h]
00007FF603BD42D6  movaps       xmmword ptr [rbp+15B0h], xmm0
00007FF603BD42DD  lea          rcx, [rbp+E30h]
00007FF603BD42E4  lea          rdx, [rbp+15B0h]
00007FF603BD42EB  call         float[4] fabled_math::linear::vec3::Vector3::to_primitive (00007FF603BDAC50h)
00007FF603BD42F0  xorps        xmm0, xmm0
00007FF603BD42F3  movss        dword ptr [rbp+E40h], xmm0
00007FF603BD42FB  xorps        xmm0, xmm0
00007FF603BD42FE  movss        dword ptr [rbp+E44h], xmm0
00007FF603BD4306  mov          rax, qword ptr [rbp+C28h]
00007FF603BD430D  mov          qword ptr [rbp+E48h], rax
00007FF603BD4314  mov          rax, qword ptr [rbp+C30h]
00007FF603BD431B  mov          qword ptr [rbp+E50h], rax
00007FF603BD4322  lea          rax, [rbp+E58h]
00007FF603BD4329  lea          rcx, [rbp+E58h]
00007FF603BD4330  add          rcx, 10h
00007FF603BD4334  mov          qword ptr [rbp-30h], rcx
00007FF603BD4338  mov          qword ptr [rbp-28h], rax
00007FF603BD433C  mov          rcx, qword ptr [rbp-30h]
00007FF603BD4340  mov          rax, qword ptr [rbp-28h]
00007FF603BD4344  mov          qword ptr [rbp-38h], rax
00007FF603BD4348  cmp          rax, rcx
00007FF603BD434B  je           struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+BB2h (00007FF603BD4362h)
00007FF603BD434D  mov          rax, qword ptr [rbp-38h]
00007FF603BD4351  xorps        xmm0, xmm0
00007FF603BD4354  movss        dword ptr [rax], xmm0
00007FF603BD4358  add          rax, 4h
00007FF603BD435C  mov          qword ptr [rbp-28h], rax
00007FF603BD4360  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+B8Ch (00007FF603BD433Ch)
00007FF603BD4362  lea          rax, [rbp+E68h]
00007FF603BD4369  lea          rcx, [rbp+E68h]
00007FF603BD4370  add          rcx, 10h
00007FF603BD4374  mov          qword ptr [rbp-48h], rcx
00007FF603BD4378  mov          qword ptr [rbp-40h], rax
00007FF603BD437C  mov          rcx, qword ptr [rbp-48h]
00007FF603BD4380  mov          rax, qword ptr [rbp-40h]
00007FF603BD4384  mov          qword ptr [rbp-50h], rax
00007FF603BD4388  cmp          rax, rcx
00007FF603BD438B  je           struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+BF2h (00007FF603BD43A2h)
00007FF603BD438D  mov          rax, qword ptr [rbp-50h]
00007FF603BD4391  xorps        xmm0, xmm0
00007FF603BD4394  movss        dword ptr [rax], xmm0
00007FF603BD4398  add          rax, 4h
00007FF603BD439C  mov          qword ptr [rbp-40h], rax
00007FF603BD43A0  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+BCCh (00007FF603BD437Ch)
00007FF603BD43A2  mov          rax, qword ptr [rbp+E58h]
00007FF603BD43A9  mov          qword ptr [rbp+DE8h], rax
00007FF603BD43B0  mov          rax, qword ptr [rbp+E60h]
00007FF603BD43B7  mov          qword ptr [rbp+DF0h], rax
00007FF603BD43BE  mov          rax, qword ptr [rbp+E68h]
00007FF603BD43C5  mov          qword ptr [rbp+DF8h], rax
00007FF603BD43CC  mov          rax, qword ptr [rbp+E70h]
00007FF603BD43D3  mov          qword ptr [rbp+E00h], rax
00007FF603BD43DA  mov          rax, qword ptr [rbp+E30h]
00007FF603BD43E1  mov          qword ptr [rbp+E08h], rax
00007FF603BD43E8  mov          rax, qword ptr [rbp+E38h]
00007FF603BD43EF  mov          qword ptr [rbp+E10h], rax
00007FF603BD43F6  mov          rax, qword ptr [rbp+E48h]
00007FF603BD43FD  mov          qword ptr [rbp+E18h], rax
00007FF603BD4404  mov          rax, qword ptr [rbp+E50h]
00007FF603BD440B  mov          qword ptr [rbp+E20h], rax
00007FF603BD4412  mov          rax, qword ptr [rbp+E40h]
00007FF603BD4419  mov          qword ptr [rbp+E28h], rax
00007FF603BD4420  lea          rcx, [rbp+F98h]
00007FF603BD4427  lea          rdx, [rbp+C38h]
00007FF603BD442E  mov          r8d, 48h
00007FF603BD4434  call         memcpy (00007FF604CA5954h)
00007FF603BD4439  lea          rcx, [rbp+FE0h]
00007FF603BD4440  lea          rdx, [rbp+CC8h]
00007FF603BD4447  mov          r8d, 48h
00007FF603BD444D  call         memcpy (00007FF604CA5954h)
00007FF603BD4452  lea          rcx, [rbp+1028h]
00007FF603BD4459  lea          rdx, [rbp+D58h]
00007FF603BD4460  mov          r8d, 48h
00007FF603BD4466  call         memcpy (00007FF604CA5954h)
00007FF603BD446B  lea          rcx, [rbp+1070h]
00007FF603BD4472  lea          rdx, [rbp+DE8h]
00007FF603BD4479  mov          r8d, 48h
00007FF603BD447F  call         memcpy (00007FF604CA5954h)
00007FF603BD4484  lea          rcx, [rbp+E78h]
00007FF603BD448B  lea          rdx, [rbp+F98h]
00007FF603BD4492  mov          r8d, 48h
00007FF603BD4498  call         memcpy (00007FF604CA5954h)
00007FF603BD449D  lea          rcx, [rbp+E78h]
00007FF603BD44A4  add          rcx, 48h
00007FF603BD44A8  lea          rdx, [rbp+FE0h]
00007FF603BD44AF  mov          r8d, 48h
00007FF603BD44B5  call         memcpy (00007FF604CA5954h)
00007FF603BD44BA  lea          rcx, [rbp+E78h]
00007FF603BD44C1  add          rcx, 90h
00007FF603BD44C8  lea          rdx, [rbp+1028h]
00007FF603BD44CF  mov          r8d, 48h
00007FF603BD44D5  call         memcpy (00007FF604CA5954h)
00007FF603BD44DA  lea          rcx, [rbp+E78h]
00007FF603BD44E1  add          rcx, D8h
00007FF603BD44E8  lea          rdx, [rbp+1070h]
00007FF603BD44EF  mov          r8d, 48h
00007FF603BD44F5  call         memcpy (00007FF604CA5954h)
00007FF603BD44FA  mov          rax, qword ptr [rbp+D8h]
00007FF603BD4501  shl          rax, 2h
00007FF603BD4505  mov          qword ptr [rbp+15C8h], rax
00007FF603BD450C  mov          qword ptr [rbp+10D0h], rax
00007FF603BD4513  mov          rdx, qword ptr [rbp+10D0h]
00007FF603BD451A  lea          rcx, [rbp+110h]
00007FF603BD4521  lea          r8, [00007FF604D58E60h]
00007FF603BD4528  call         struct slice$<fabled_render::mesh::container::vertex::Vertex> core::array::impl$16::index_mut<fabled_render::mesh::container::vertex::Vertex,core::ops::range::RangeFrom<usize>,24> (00007FF603BD9E20h)
00007FF603BD452D  mov          qword ptr [rbp-58h], rax
00007FF603BD4531  mov          r8, rdx
00007FF603BD4534  mov          rdx, qword ptr [rbp-58h]
00007FF603BD4538  lea          rcx, [rbp+10B8h]
00007FF603BD453F  lea          r9, [00007FF604D58E78h]
00007FF603BD4546  call         struct tuple$<ref_mut$<array$<fabled_render::mesh::container::vertex::Vertex,4> >,slice$<fabled_render::mesh::container::vertex::Vertex> > core::slice::impl$0::split_array_mut<fabled_render::mesh::container::vertex::Vertex,4> (00007FF603BD9FB0h)
00007FF603BD454B  mov          rcx, qword ptr [rbp+10B8h]
00007FF603BD4552  mov          qword ptr [rbp+15D0h], rcx
00007FF603BD4559  lea          r8, [rbp+E78h]
00007FF603BD4560  mov          r9d, 4h
00007FF603BD4566  lea          rax, [00007FF604D58E90h]
00007FF603BD456D  mov          rdx, r9
00007FF603BD4570  mov          qword ptr [rsp+20h], rax
00007FF603BD4575  call         void core::slice::impl$0::copy_from_slice<fabled_render::mesh::container::vertex::Vertex> (00007FF603BD9EF0h)
00007FF603BD457A  jmp          struct fabled_render::mesh::container::mesh_data::Mesh fabled_render::mesh::primitive::cube::impl$2::from+F4h (00007FF603BD38A4h)
00007FF603BD457F  lea          rcx, [rbp+10F0h]
00007FF603BD4586  add          rcx, 8h
00007FF603BD458A  lea          rdx, [rbp+1200h]
00007FF603BD4591  mov          r8d, 108h
00007FF603BD4597  call         memcpy (00007FF604CA5954h)
00007FF603BD459C  mov          rcx, qword ptr [rbp+F0h]
00007FF603BD45A3  mov          qword ptr [rbp+10F0h], 1h
00007FF603BD45AE  mov          rax, qword ptr [rbp+10D8h]
00007FF603BD45B5  mov          qword ptr [rcx], rax
00007FF603BD45B8  mov          rax, qword ptr [rbp+10E0h]
00007FF603BD45BF  mov          qword ptr [rcx+8h], rax
00007FF603BD45C3  mov          rax, qword ptr [rbp+10E8h]
00007FF603BD45CA  mov          qword ptr [rcx+10h], rax
00007FF603BD45CE  add          rcx, 18h
00007FF603BD45D2  lea          rdx, [rbp+10F0h]
00007FF603BD45D9  mov          r8d, 110h
00007FF603BD45DF  call         memcpy (00007FF604CA5954h)
00007FF603BD45E4  mov          rax, qword ptr [rbp+F8h]
00007FF603BD45EB  add          rsp, 1660h
00007FF603BD45F2  pop          rbp
00007FF603BD45F3  ret
00007FF603BD45F4  nop          word ptr cs:[rax+rax*1], ax
00007FF603BD45FE  nop
00007FF603BD4600  mov          qword ptr [rsp+10h], rdx
00007FF603BD4605  push         rbp
00007FF603BD4606  sub          rsp, 30h
00007FF603BD460A  lea          rbp, [rdx+80h]
00007FF603BD4611  lea          rcx, [rbp+10D8h]
00007FF603BD4618  call         void core::ptr::drop_in_place<alloc::vec::Vec<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> > (00007FF603BD8F20h)
00007FF603BD461D  nop
00007FF603BD461E  add          rsp, 30h
00007FF603BD4622  pop          rbp
00007FF603BD4623  ret
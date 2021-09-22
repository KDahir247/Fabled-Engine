# --------------- Material Parser Dissassembly -------------------
// parse_material
00007FF69345EF33  mov          rax, qword ptr [rsp+1E0h]
00007FF69345EF3B  mov          qword ptr [rsp+1A0h], rax
00007FF69345EF43  movups       xmm0, xmmword ptr [rsp+1D0h]
00007FF69345EF4B  movaps       xmmword ptr [rsp+190h], xmm0
00007FF69345EF53  mov          byte ptr [rsp+128h], 2h
00007FF69345EF5B  lea          rcx, [rsp+30h]
00007FF69345EF60  lea          rdx, [rsp+190h]
00007FF69345EF68  lea          r8, [rsp+128h]
00007FF69345EF70  call         static union enum$<core::result::Result<naga::Module, anyhow::Error>> fabled_render::shader::parser::shader_parser::parse_shader<alloc::string::String> (00007FF693451000h)
00007FF69345EF75  cmp          dword ptr [rsp+30h], 1h
00007FF69345EF7A  mov          r14, qword ptr [rsp+38h]
00007FF69345EF7F  je           static void Fabled_Engine::main+102Bh (00007FF69345FE3Bh)
00007FF69345EF85  movups       xmm0, xmmword ptr [rsp+40h]
00007FF69345EF8A  movaps       xmmword ptr [rsp+240h], xmm0
00007FF69345EF92  mov          rax, qword ptr [rsp+50h]
00007FF69345EF97  mov          qword ptr [rsp+C8h], rax
00007FF69345EF9F  mov          rax, qword ptr [rsp+58h]
00007FF69345EFA4  mov          qword ptr [rsp+1C0h], rax
00007FF69345EFAC  mov          rax, qword ptr [rsp+60h]
00007FF69345EFB1  mov          qword ptr [rsp+D8h], rax
00007FF69345EFB9  mov          rcx, qword ptr [rsp+68h]
00007FF69345EFBE  mov          qword ptr [rsp+E8h], rcx
00007FF69345EFC6  mov          rax, qword ptr [rsp+70h]
00007FF69345EFCB  mov          qword ptr [rsp+D0h], rax
00007FF69345EFD3  mov          rdx, qword ptr [rsp+78h]
00007FF69345EFD8  mov          rax, qword ptr [rsp+80h]
00007FF69345EFE0  mov          qword ptr [rsp+C0h], rax
00007FF69345EFE8  mov          rax, qword ptr [rsp+88h]
00007FF69345EFF0  mov          qword ptr [rsp+100h], rax
00007FF69345EFF8  mov          rax, qword ptr [rsp+90h]
00007FF69345F000  mov          qword ptr [rsp+F0h], rax
00007FF69345F008  mov          rax, qword ptr [rsp+98h]
00007FF69345F010  mov          qword ptr [rsp+E0h], rax
00007FF69345F018  mov          rax, qword ptr [rsp+A0h]
00007FF69345F020  mov          qword ptr [rsp+108h], rax
00007FF69345F028  mov          rax, qword ptr [rsp+A8h]
00007FF69345F030  mov          qword ptr [rsp+F8h], rax
00007FF69345F038  mov          qword ptr [rsp+178h], r14
00007FF69345F040  movups       xmmword ptr [rsp+180h], xmm0
00007FF69345F048  mov          qword ptr [rsp+1B8h], rdx
00007FF69345F050  imul         rax, rdx, 38h
00007FF69345F054  mov          qword ptr [rsp+28h], rax
00007FF69345F059  lea          rbx, [rcx+rax*1]
00007FF69345F05D  lea          r13, [rsp+178h]
00007FF69345F065  mov          qword ptr [rsp+1E8h], rbx
00007FF69345F06D  jmp          static void Fabled_Engine::main+281h (00007FF69345F091h)
00007FF69345F06F  nop
00007FF69345F070  mov          rax, qword ptr [r15]
00007FF69345F073  mov          dword ptr [rax+rdx*8], edi
00007FF69345F076  mov          dword ptr [rax+rdx*8+4h], esi
00007FF69345F07A  add          rdx, 1h
00007FF69345F07E  mov          qword ptr [r14], rdx
00007FF69345F081  mov          rbx, qword ptr [rsp+1E8h]
00007FF69345F089  mov          rcx, qword ptr [rsp+1F0h]
00007FF69345F091  cmp          rcx, rbx
00007FF69345F094  je           static void Fabled_Engine::main+65Eh (00007FF69345F46Eh)
00007FF69345F09A  mov          rdi, rcx
00007FF69345F09D  add          rcx, 38h
00007FF69345F0A1  cmp          dword ptr [rdi+18h], 1h
00007FF69345F0A5  jne          static void Fabled_Engine::main+281h (00007FF69345F091h)
00007FF69345F0A7  mov          qword ptr [rsp+1F0h], rcx
00007FF69345F0AF  mov          esi, dword ptr [rdi+24h]
00007FF69345F0B2  mov          rcx, r13
00007FF69345F0B5  call         static struct slice<u8> alloc::vec::{{impl}}::deref<u8,alloc::alloc::Global> (00007FF69345EB40h)
00007FF69345F0BA  add          rsi, FFFFFFFFFFFFFFFFh
00007FF69345F0BE  cmp          rdx, rsi
00007FF69345F0C1  jbe          static void Fabled_Engine::main+D87h (00007FF69345FB97h)
00007FF69345F0C7  test         rax, rax
00007FF69345F0CA  je           static void Fabled_Engine::main+D87h (00007FF69345FB97h)
00007FF69345F0D0  mov          qword ptr [rsp+1C8h], rdi
00007FF69345F0D8  cmp          dword ptr [rdi+18h], 1h
00007FF69345F0DC  jne          static void Fabled_Engine::main+E33h (00007FF69345FC43h)
00007FF69345F0E2  imul         rcx, rsi, 38h
00007FF69345F0E6  mov          qword ptr [rsp+118h], rax
00007FF69345F0EE  mov          qword ptr [rsp+110h], rcx
00007FF69345F0F6  cmp          byte ptr [rax+rcx*1+18h], 6h
00007FF69345F0FB  jne          static void Fabled_Engine::main+550h (00007FF69345F360h)
00007FF69345F101  mov          rcx, qword ptr [rsp+118h]
00007FF69345F109  mov          rdx, qword ptr [rsp+110h]
00007FF69345F111  lea          rax, [rcx+rdx*1]
00007FF69345F115  cmp          byte ptr [rax+19h], 0h
00007FF69345F119  je           static void Fabled_Engine::main+4D5h (00007FF69345F2E5h)
00007FF69345F11F  mov          rax, qword ptr [rcx+rdx*1+30h]
00007FF69345F124  test         rax, rax
00007FF69345F127  je           static void Fabled_Engine::main+550h (00007FF69345F360h)
00007FF69345F12D  mov          rdx, qword ptr [rsp+1C8h]
00007FF69345F135  mov          ecx, dword ptr [rdx+1Ch]
00007FF69345F138  mov          dword ptr [rsp+124h], ecx
00007FF69345F13F  mov          ecx, dword ptr [rdx+20h]
00007FF69345F142  mov          dword ptr [rsp+120h], ecx
00007FF69345F149  mov          rcx, qword ptr [rsp+118h]
00007FF69345F151  mov          rdx, qword ptr [rsp+110h]
00007FF69345F159  mov          rbp, qword ptr [rcx+rdx*1+20h]
00007FF69345F15E  shl          rax, 3h
00007FF69345F162  lea          rdi, [rax+rax*4]
00007FF69345F166  xor          ebx, ebx
00007FF69345F168  mov          qword ptr [rsp+1F8h], rdi
00007FF69345F170  jmp          static void Fabled_Engine::main+39Fh (00007FF69345F1AFh)
00007FF69345F172  nop          word ptr cs:[rax+rax*1], ax
00007FF69345F17C  nop          dword ptr [rax], eax
00007FF69345F180  mov          rax, qword ptr [rsi]
00007FF69345F183  mov          dword ptr [rax+rdx*8], r14d
00007FF69345F187  mov          dword ptr [rax+rdx*8+4h], r15d
00007FF69345F18C  add          rdx, 1h
00007FF69345F190  mov          qword ptr [r13], rdx
00007FF69345F194  mov          r13, rbp
00007FF69345F197  mov          rbp, rdi
00007FF69345F19A  mov          rdi, qword ptr [rsp+1F8h]
00007FF69345F1A2  add          rbx, 28h
00007FF69345F1A6  cmp          rdi, rbx
00007FF69345F1A9  je           static void Fabled_Engine::main+550h (00007FF69345F360h)
00007FF69345F1AF  mov          esi, dword ptr [rbp+rbx*1+18h]
00007FF69345F1B3  mov          rcx, r13
00007FF69345F1B6  call         static struct slice<u8> alloc::vec::{{impl}}::deref<u8,alloc::alloc::Global> (00007FF69345EB40h)
00007FF69345F1BB  add          rsi, FFFFFFFFFFFFFFFFh
00007FF69345F1BF  cmp          rdx, rsi
00007FF69345F1C2  jbe          static void Fabled_Engine::main+392h (00007FF69345F1A2h)
00007FF69345F1C4  test         rax, rax
00007FF69345F1C7  je           static void Fabled_Engine::main+392h (00007FF69345F1A2h)
00007FF69345F1C9  imul         rcx, rsi, 38h
00007FF69345F1CD  lea          rdx, [rax+rcx*1]
00007FF69345F1D1  add          rdx, 18h
00007FF69345F1D5  lea          rcx, [rsp+128h]
00007FF69345F1DD  call         static union enum$<fabled_render::material::material_graph::material_target::MaterialTarget> fabled_render::material::material_graph::material_target::{{impl}}::from (00007FF69346E2D0h)
00007FF69345F1E2  mov          rax, qword ptr [rsp+128h]
00007FF69345F1EA  lea          rcx, [00007FF6934602B8h]
00007FF69345F1F1  movsxd       rax, dword ptr [rcx+rax*4]
00007FF69345F1F5  add          rax, rcx
00007FF69345F1F8  xor          r12d, r12d
00007FF69345F1FB  jmp          rax
00007FF69345F1FD  mov          r12d, 1h
00007FF69345F203  jmp          static void Fabled_Engine::main+410h (00007FF69345F220h)
00007FF69345F205  mov          r12d, 2h
00007FF69345F20B  jmp          static void Fabled_Engine::main+410h (00007FF69345F220h)
00007FF69345F20D  mov          r12d, 4h
00007FF69345F213  nop          word ptr cs:[rax+rax*1], ax
00007FF69345F21D  nop          dword ptr [rax], eax
00007FF69345F220  movups       xmm0, xmmword ptr [rsp+168h]
00007FF69345F228  movaps       xmmword ptr [rsp+70h], xmm0
00007FF69345F22D  movups       xmm0, xmmword ptr [rsp+128h]
00007FF69345F235  movups       xmm1, xmmword ptr [rsp+138h]
00007FF69345F23D  movups       xmm2, xmmword ptr [rsp+148h]
00007FF69345F245  movups       xmm3, xmmword ptr [rsp+158h]
00007FF69345F24D  movaps       xmmword ptr [rsp+60h], xmm3
00007FF69345F252  movaps       xmmword ptr [rsp+50h], xmm2
00007FF69345F257  movaps       xmmword ptr [rsp+40h], xmm1
00007FF69345F25C  movaps       xmmword ptr [rsp+30h], xmm0
00007FF69345F261  mov          eax, dword ptr [rsp+124h]
00007FF69345F268  mov          dword ptr [rsp+80h], eax
00007FF69345F26F  mov          eax, dword ptr [rsp+120h]
00007FF69345F276  mov          dword ptr [rsp+84h], eax
00007FF69345F27D  lea          rcx, [rsp+218h]
00007FF69345F285  lea          rdx, [rsp+30h]
00007FF69345F28A  call         static struct fabled_render::material::material_graph::material_key::MaterialKey slotmap::normal::SlotMap<fabled_render::material::material_graph::material_key::MaterialKey, fabled_render::material::material_graph::material_node::MaterialNode>::insert<fabled_render::material::material_graph::material_key::MaterialKey,fabled_render::material::material_graph::material_node::MaterialNode> (00007FF6934594C0h)
00007FF69345F28F  mov          r15d, edx
00007FF69345F292  mov          rdx, qword ptr [rsp+210h]
00007FF69345F29A  cmp          rdx, r12
00007FF69345F29D  jbe          static void Fabled_Engine::main+D54h (00007FF69345FB64h)
00007FF69345F2A3  mov          r14d, eax
00007FF69345F2A6  mov          rdi, rbp
00007FF69345F2A9  mov          rbp, r13
00007FF69345F2AC  mov          rax, qword ptr [rsp+200h]
00007FF69345F2B4  shl          r12, 5h
00007FF69345F2B8  lea          rsi, [rax+r12*1]
00007FF69345F2BC  lea          r13, [rax+r12*1]
00007FF69345F2C0  add          r13, 10h
00007FF69345F2C4  mov          rdx, qword ptr [rax+r12*1+10h]
00007FF69345F2C9  cmp          rdx, qword ptr [rax+r12*1+8h]
00007FF69345F2CE  jne          static void Fabled_Engine::main+370h (00007FF69345F180h)
00007FF69345F2D4  mov          rcx, rsi
00007FF69345F2D7  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::material::material_graph::material_key::MaterialKey,alloc::alloc::Global> (00007FF693556260h)
00007FF69345F2DC  mov          rdx, qword ptr [r13]
00007FF69345F2E0  jmp          static void Fabled_Engine::main+370h (00007FF69345F180h)
00007FF69345F2E5  cmp          qword ptr [rax], 0h
00007FF69345F2E9  je           static void Fabled_Engine::main+11F7h (00007FF693460007h)
00007FF69345F2EF  mov          qword ptr [rsp+190h], rax
00007FF69345F2F7  lea          rax, [rsp+190h]
00007FF69345F2FF  mov          qword ptr [rsp+128h], rax
00007FF69345F307  lea          rax, [static core::result::Result core::fmt::{{impl}}::fmt<alloc::string::String> (00007FF6934DA7A0h)]
00007FF69345F30E  mov          qword ptr [rsp+130h], rax
00007FF69345F316  lea          rax, [00007FF6935627C0h]
00007FF69345F31D  mov          qword ptr [rsp+30h], rax
00007FF69345F322  mov          qword ptr [rsp+38h], 2h
00007FF69345F32B  mov          qword ptr [rsp+40h], 0h
00007FF69345F334  lea          rax, [rsp+128h]
00007FF69345F33C  mov          qword ptr [rsp+50h], rax
00007FF69345F341  mov          qword ptr [rsp+58h], 1h
00007FF69345F34A  lea          rcx, [rsp+30h]
00007FF69345F34F  call         static void std::io::stdio::_print (00007FF6935502E0h)
00007FF69345F354  nop          word ptr cs:[rax+rax*1], ax
00007FF69345F35E  nop
00007FF69345F360  mov          rax, qword ptr [rsp+118h]
00007FF69345F368  mov          rcx, qword ptr [rsp+110h]
00007FF69345F370  lea          rdx, [rax+rcx*1]
00007FF69345F374  add          rdx, 18h
00007FF69345F378  mov          rax, qword ptr [rsp+1C8h]
00007FF69345F380  mov          esi, dword ptr [rax+1Ch]
00007FF69345F383  mov          edi, dword ptr [rax+20h]
00007FF69345F386  lea          rcx, [rsp+128h]
00007FF69345F38E  call         static union enum$<fabled_render::material::material_graph::material_target::MaterialTarget> fabled_render::material::material_graph::material_target::{{impl}}::from (00007FF69346E2D0h)
00007FF69345F393  mov          rax, qword ptr [rsp+128h]
00007FF69345F39B  lea          rcx, [00007FF6934602ECh]
00007FF69345F3A2  movsxd       rax, dword ptr [rcx+rax*4]
00007FF69345F3A6  add          rax, rcx
00007FF69345F3A9  xor          ebx, ebx
00007FF69345F3AB  jmp          rax
00007FF69345F3AD  mov          ebx, 1h
00007FF69345F3B2  jmp          static void Fabled_Engine::main+5B0h (00007FF69345F3C0h)
00007FF69345F3B4  mov          ebx, 2h
00007FF69345F3B9  jmp          static void Fabled_Engine::main+5B0h (00007FF69345F3C0h)
00007FF69345F3BB  mov          ebx, 4h
00007FF69345F3C0  movups       xmm0, xmmword ptr [rsp+168h]
00007FF69345F3C8  movaps       xmmword ptr [rsp+70h], xmm0
00007FF69345F3CD  movups       xmm0, xmmword ptr [rsp+128h]
00007FF69345F3D5  movups       xmm1, xmmword ptr [rsp+138h]
00007FF69345F3DD  movups       xmm2, xmmword ptr [rsp+148h]
00007FF69345F3E5  movups       xmm3, xmmword ptr [rsp+158h]
00007FF69345F3ED  movaps       xmmword ptr [rsp+60h], xmm3
00007FF69345F3F2  movaps       xmmword ptr [rsp+50h], xmm2
00007FF69345F3F7  movaps       xmmword ptr [rsp+40h], xmm1
00007FF69345F3FC  movaps       xmmword ptr [rsp+30h], xmm0
00007FF69345F401  mov          dword ptr [rsp+80h], esi
00007FF69345F408  mov          dword ptr [rsp+84h], edi
00007FF69345F40F  lea          rcx, [rsp+218h]
00007FF69345F417  lea          rdx, [rsp+30h]
00007FF69345F41C  call         static struct fabled_render::material::material_graph::material_key::MaterialKey slotmap::normal::SlotMap<fabled_render::material::material_graph::material_key::MaterialKey, fabled_render::material::material_graph::material_node::MaterialNode>::insert<fabled_render::material::material_graph::material_key::MaterialKey,fabled_render::material::material_graph::material_node::MaterialNode> (00007FF6934594C0h)
00007FF69345F421  mov          esi, edx
00007FF69345F423  mov          rdx, qword ptr [rsp+210h]
00007FF69345F42B  cmp          rdx, rbx
00007FF69345F42E  jbe          static void Fabled_Engine::main+D65h (00007FF69345FB75h)
00007FF69345F434  mov          edi, eax
00007FF69345F436  mov          rax, qword ptr [rsp+200h]
00007FF69345F43E  shl          rbx, 5h
00007FF69345F442  lea          r15, [rax+rbx*1]
00007FF69345F446  lea          r14, [rax+rbx*1]
00007FF69345F44A  add          r14, 10h
00007FF69345F44E  mov          rdx, qword ptr [rax+rbx*1+10h]
00007FF69345F453  cmp          rdx, qword ptr [rax+rbx*1+8h]
00007FF69345F458  jne          static void Fabled_Engine::main+260h (00007FF69345F070h)
00007FF69345F45E  mov          rcx, r15
00007FF69345F461  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::material::material_graph::material_key::MaterialKey,alloc::alloc::Global> (00007FF693556260h)
00007FF69345F466  mov          rdx, qword ptr [r14]
00007FF69345F469  jmp          static void Fabled_Engine::main+260h (00007FF69345F070h)
00007FF69345F46E  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F475  test         rcx, rcx
00007FF69345F478  je           static void Fabled_Engine::main+674h (00007FF69345F484h)
00007FF69345F47A  mov          rdi, qword ptr [rsp+D8h]
00007FF69345F482  jmp          static void Fabled_Engine::main+694h (00007FF69345F4A4h)
00007FF69345F484  call         GetProcessHeap (00007FF693553B8Ch)
00007FF69345F489  test         rax, rax
00007FF69345F48C  mov          rdi, qword ptr [rsp+D8h]
00007FF69345F494  je           static void Fabled_Engine::main+12E3h (00007FF6934600F3h)
00007FF69345F49A  mov          rcx, rax
00007FF69345F49D  mov          qword ptr [00007FF6935801D8h], rax
00007FF69345F4A4  mov          r8d, 2h
00007FF69345F4AA  xor          edx, edx
00007FF69345F4AC  call         HeapAlloc (00007FF693553B92h)
00007FF69345F4B1  test         rax, rax
00007FF69345F4B4  je           static void Fabled_Engine::main+12E3h (00007FF6934600F3h)
00007FF69345F4BA  mov          rsi, rax
00007FF69345F4BD  mov          word ptr [rax], A0Dh
00007FF69345F4C2  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F4C9  test         rcx, rcx
00007FF69345F4CC  jne          static void Fabled_Engine::main+6D6h (00007FF69345F4E6h)
00007FF69345F4CE  call         GetProcessHeap (00007FF693553B8Ch)
00007FF69345F4D3  test         rax, rax
00007FF69345F4D6  je           static void Fabled_Engine::main+12F4h (00007FF693460104h)
00007FF69345F4DC  mov          rcx, rax
00007FF69345F4DF  mov          qword ptr [00007FF6935801D8h], rax
00007FF69345F4E6  mov          r8d, 4h
00007FF69345F4EC  xor          edx, edx
00007FF69345F4EE  call         HeapAlloc (00007FF693553B92h)
00007FF69345F4F3  test         rax, rax
00007FF69345F4F6  je           static void Fabled_Engine::main+12F4h (00007FF693460104h)
00007FF69345F4FC  mov          dword ptr [rax], 20202020h
00007FF69345F502  mov          rcx, qword ptr [00007FF693578468h]
00007FF69345F509  mov          qword ptr [rsp+128h], rcx
00007FF69345F511  xorps        xmm0, xmm0
00007FF69345F514  movups       xmmword ptr [rsp+130h], xmm0
00007FF69345F51C  mov          cl, byte ptr [rsp+1B7h]
00007FF69345F523  mov          edx, dword ptr [rsp+1B3h]
00007FF69345F52A  mov          rbp, qword ptr [00007FF693578310h]
00007FF69345F531  mov          rbx, qword ptr [rsp+128h]
00007FF69345F539  mov          qword ptr [rsp+30h], rbx
00007FF69345F53E  mov          rbx, qword ptr [rsp+130h]
00007FF69345F546  mov          qword ptr [rsp+38h], rbx
00007FF69345F54B  mov          rbx, qword ptr [rsp+138h]
00007FF69345F553  mov          qword ptr [rsp+40h], rbx
00007FF69345F558  mov          word ptr [rsp+89h], 101h
00007FF69345F562  mov          dword ptr [rsp+8Bh], edx
00007FF69345F569  mov          byte ptr [rsp+8Fh], cl
00007FF69345F570  movaps       xmmword ptr [rsp+A0h], xmm0
00007FF69345F578  mov          qword ptr [rsp+48h], 5h
00007FF69345F581  mov          qword ptr [rsp+50h], rsi
00007FF69345F586  movaps       xmm0, xmmword ptr [__xmm@00000000000000020000000000000002 (00007FF6935615E0h)]
00007FF69345F58D  movups       xmmword ptr [rsp+58h], xmm0
00007FF69345F592  mov          qword ptr [rsp+68h], rax
00007FF69345F597  movaps       xmm0, xmmword ptr [__xmm@00000000000000040000000000000004 (00007FF6935615C0h)]
00007FF69345F59E  movaps       xmmword ptr [rsp+70h], xmm0
00007FF69345F5A3  mov          qword ptr [rsp+80h], 0h
00007FF69345F5AF  mov          byte ptr [rsp+88h], 0h
00007FF69345F5B7  mov          qword ptr [rsp+90h], 0h
00007FF69345F5C3  mov          qword ptr [rsp+98h], rbp
00007FF69345F5CB  mov          word ptr [rsp+B0h], 200h
00007FF69345F5D5  lea          rcx, [rsp+128h]
00007FF69345F5DD  lea          rdx, [rsp+200h]
00007FF69345F5E5  lea          r8, [rsp+30h]
00007FF69345F5EA  call         static union enum$<core::result::Result<tuple<>, ron::error::Error>, 0, 32, Err> fabled_render::shader::parser::material_parser::_::{{impl}}::serialize<mut ron::ser::Serializer<alloc::vec::Vec<u8, alloc::alloc::Global>>*> (00007FF69345B380h)
00007FF69345F5EF  mov          ebp, dword ptr [rsp+128h]
00007FF69345F5F6  cmp          rbp, 21h
00007FF69345F5FA  jne          static void Fabled_Engine::main+1305h (00007FF693460115h)
00007FF69345F600  mov          r15, qword ptr [rsp+30h]
00007FF69345F605  mov          r14, qword ptr [rsp+38h]
00007FF69345F60A  mov          rsi, qword ptr [rsp+40h]
00007FF69345F60F  lea          rcx, [rsp+128h]
00007FF69345F617  mov          rdx, r15
00007FF69345F61A  mov          r8, rsi
00007FF69345F61D  call         static void core::str::converts::from_utf8 (00007FF69346BB00h)
00007FF69345F622  cmp          dword ptr [rsp+128h], 1h
00007FF69345F62A  je           static void Fabled_Engine::main+13FBh (00007FF69346020Bh)
00007FF69345F630  cmp          byte ptr [rsp+88h], 2h
00007FF69345F638  mov          rbx, qword ptr [rsp+E8h]
00007FF69345F640  mov          rcx, qword ptr [rsp+28h]
00007FF69345F645  je           static void Fabled_Engine::main+8A3h (00007FF69345F6B3h)
00007FF69345F647  cmp          qword ptr [rsp+58h], 0h
00007FF69345F64D  je           static void Fabled_Engine::main+857h (00007FF69345F667h)
00007FF69345F64F  mov          r8, qword ptr [rsp+50h]
00007FF69345F654  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F65B  xor          edx, edx
00007FF69345F65D  call         HeapFree (00007FF693553B98h)
00007FF69345F662  mov          rcx, qword ptr [rsp+28h]
00007FF69345F667  cmp          qword ptr [rsp+70h], 0h
00007FF69345F66D  je           static void Fabled_Engine::main+877h (00007FF69345F687h)
00007FF69345F66F  mov          r8, qword ptr [rsp+68h]
00007FF69345F674  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F67B  xor          edx, edx
00007FF69345F67D  call         HeapFree (00007FF693553B98h)
00007FF69345F682  mov          rcx, qword ptr [rsp+28h]
00007FF69345F687  mov          rax, qword ptr [rsp+A0h]
00007FF69345F68F  shl          rax, 3h
00007FF69345F693  test         rax, rax
00007FF69345F696  je           static void Fabled_Engine::main+8A3h (00007FF69345F6B3h)
00007FF69345F698  mov          r8, qword ptr [rsp+98h]
00007FF69345F6A0  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F6A7  xor          edx, edx
00007FF69345F6A9  call         HeapFree (00007FF693553B98h)
00007FF69345F6AE  mov          rcx, qword ptr [rsp+28h]
00007FF69345F6B3  cmp          qword ptr [rsp+1B8h], 0h
00007FF69345F6BC  mov          rax, qword ptr [rsp+D0h]
00007FF69345F6C4  je           static void Fabled_Engine::main+8F7h (00007FF69345F707h)
00007FF69345F6C6  xor          esi, esi
00007FF69345F6C8  jmp          static void Fabled_Engine::main+8C9h (00007FF69345F6D9h)
00007FF69345F6CA  nop          word ptr [rax+rax*1], ax
00007FF69345F6D0  add          rsi, 38h
00007FF69345F6D4  cmp          rcx, rsi
00007FF69345F6D7  je           static void Fabled_Engine::main+8F7h (00007FF69345F707h)
00007FF69345F6D9  mov          r8, qword ptr [rbx+rsi*1]
00007FF69345F6DD  test         r8, r8
00007FF69345F6E0  je           static void Fabled_Engine::main+8C0h (00007FF69345F6D0h)
00007FF69345F6E2  cmp          qword ptr [rbx+rsi*1+8h], 0h
00007FF69345F6E8  je           static void Fabled_Engine::main+8C0h (00007FF69345F6D0h)
00007FF69345F6EA  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F6F1  xor          edx, edx
00007FF69345F6F3  call         HeapFree (00007FF693553B98h)
00007FF69345F6F8  mov          rcx, qword ptr [rsp+28h]
00007FF69345F6FD  mov          rax, qword ptr [rsp+D0h]
00007FF69345F705  jmp          static void Fabled_Engine::main+8C0h (00007FF69345F6D0h)
00007FF69345F707  test         rax, rax
00007FF69345F70A  je           static void Fabled_Engine::main+916h (00007FF69345F726h)
00007FF69345F70C  imul         rax, rax, 38h
00007FF69345F710  test         rax, rax
00007FF69345F713  je           static void Fabled_Engine::main+916h (00007FF69345F726h)
00007FF69345F715  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F71C  xor          edx, edx
00007FF69345F71E  mov          r8, rbx
00007FF69345F721  call         HeapFree (00007FF693553B98h)
00007FF69345F726  mov          rax, qword ptr [rsp+188h]
00007FF69345F72E  test         rax, rax
00007FF69345F731  je           static void Fabled_Engine::main+9FDh (00007FF69345F80Dh)
00007FF69345F737  mov          r13, qword ptr [rsp+178h]
00007FF69345F73F  imul         r12, rax, 38h
00007FF69345F743  add          r12, r13
00007FF69345F746  jmp          static void Fabled_Engine::main+949h (00007FF69345F759h)
00007FF69345F748  nop          dword ptr [rax+rax*1], eax
00007FF69345F750  cmp          r13, r12
00007FF69345F753  je           static void Fabled_Engine::main+9FDh (00007FF69345F80Dh)
00007FF69345F759  mov          rbp, r13
00007FF69345F75C  mov          r8, qword ptr [r13]
00007FF69345F760  test         r8, r8
00007FF69345F763  je           static void Fabled_Engine::main+96Ah (00007FF69345F77Ah)
00007FF69345F765  cmp          qword ptr [rbp+8h], 0h
00007FF69345F76A  je           static void Fabled_Engine::main+96Ah (00007FF69345F77Ah)
00007FF69345F76C  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F773  xor          edx, edx
00007FF69345F775  call         HeapFree (00007FF693553B98h)
00007FF69345F77A  lea          r13, [rbp+38h]
00007FF69345F77E  cmp          byte ptr [rbp+18h], 6h
00007FF69345F782  jne          static void Fabled_Engine::main+940h (00007FF69345F750h)
00007FF69345F784  mov          rax, qword ptr [rbp+30h]
00007FF69345F788  test         rax, rax
00007FF69345F78B  je           static void Fabled_Engine::main+9C0h (00007FF69345F7D0h)
00007FF69345F78D  mov          rdi, qword ptr [rbp+20h]
00007FF69345F791  shl          rax, 3h
00007FF69345F795  lea          rsi, [rax+rax*4]
00007FF69345F799  xor          ebx, ebx
00007FF69345F79B  jmp          static void Fabled_Engine::main+999h (00007FF69345F7A9h)
00007FF69345F79D  nop          dword ptr [rax], eax
00007FF69345F7A0  add          rbx, 28h
00007FF69345F7A4  cmp          rsi, rbx
00007FF69345F7A7  je           static void Fabled_Engine::main+9C0h (00007FF69345F7D0h)
00007FF69345F7A9  mov          r8, qword ptr [rdi+rbx*1]
00007FF69345F7AD  test         r8, r8
00007FF69345F7B0  je           static void Fabled_Engine::main+990h (00007FF69345F7A0h)
00007FF69345F7B2  cmp          qword ptr [rdi+rbx*1+8h], 0h
00007FF69345F7B8  je           static void Fabled_Engine::main+990h (00007FF69345F7A0h)
00007FF69345F7BA  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F7C1  xor          edx, edx
00007FF69345F7C3  call         HeapFree (00007FF693553B98h)
00007FF69345F7C8  jmp          static void Fabled_Engine::main+990h (00007FF69345F7A0h)
00007FF69345F7CA  nop          word ptr [rax+rax*1], ax
00007FF69345F7D0  mov          rax, qword ptr [rbp+28h]
00007FF69345F7D4  test         rax, rax
00007FF69345F7D7  mov          rdi, qword ptr [rsp+D8h]
00007FF69345F7DF  je           static void Fabled_Engine::main+940h (00007FF69345F750h)
00007FF69345F7E5  shl          rax, 3h
00007FF69345F7E9  lea          rax, [rax+rax*4]
00007FF69345F7ED  test         rax, rax
00007FF69345F7F0  je           static void Fabled_Engine::main+940h (00007FF69345F750h)
00007FF69345F7F6  mov          r8, qword ptr [rbp+20h]
00007FF69345F7FA  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F801  xor          edx, edx
00007FF69345F803  call         HeapFree (00007FF693553B98h)
00007FF69345F808  jmp          static void Fabled_Engine::main+940h (00007FF69345F750h)
00007FF69345F80D  mov          rax, qword ptr [rsp+180h]
00007FF69345F815  test         rax, rax
00007FF69345F818  je           static void Fabled_Engine::main+A29h (00007FF69345F839h)
00007FF69345F81A  imul         rax, rax, 38h
00007FF69345F81E  test         rax, rax
00007FF69345F821  je           static void Fabled_Engine::main+A29h (00007FF69345F839h)
00007FF69345F823  mov          r8, qword ptr [rsp+178h]
00007FF69345F82B  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F832  xor          edx, edx
00007FF69345F834  call         HeapFree (00007FF693553B98h)
00007FF69345F839  test         rdi, rdi
00007FF69345F83C  je           static void Fabled_Engine::main+AA5h (00007FF69345F8B5h)
00007FF69345F83E  shl          rdi, 6h
00007FF69345F842  mov          rax, qword ptr [rsp+C8h]
00007FF69345F84A  add          rdi, rax
00007FF69345F84D  lea          rsi, [rax+28h]
00007FF69345F851  jmp          static void Fabled_Engine::main+A5Dh (00007FF69345F86Dh)
00007FF69345F853  nop          word ptr cs:[rax+rax*1], ax
00007FF69345F85D  nop          dword ptr [rax], eax
00007FF69345F860  add          rsi, 40h
00007FF69345F864  add          rbx, 40h
00007FF69345F868  cmp          rbx, rdi
00007FF69345F86B  je           static void Fabled_Engine::main+AA5h (00007FF69345F8B5h)
00007FF69345F86D  lea          rbx, [rsi-28h]
00007FF69345F871  mov          r8, qword ptr [rsi-28h]
00007FF69345F875  test         r8, r8
00007FF69345F878  je           static void Fabled_Engine::main+A7Fh (00007FF69345F88Fh)
00007FF69345F87A  cmp          qword ptr [rsi-20h], 0h
00007FF69345F87F  je           static void Fabled_Engine::main+A7Fh (00007FF69345F88Fh)
00007FF69345F881  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F888  xor          edx, edx
00007FF69345F88A  call         HeapFree (00007FF693553B98h)
00007FF69345F88F  cmp          byte ptr [rbx+18h], 0h
00007FF69345F893  je           static void Fabled_Engine::main+A50h (00007FF69345F860h)
00007FF69345F895  mov          rax, qword ptr [rsi]
00007FF69345F898  shl          rax, 2h
00007FF69345F89C  test         rax, rax
00007FF69345F89F  je           static void Fabled_Engine::main+A50h (00007FF69345F860h)
00007FF69345F8A1  mov          r8, qword ptr [rsi-8h]
00007FF69345F8A5  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F8AC  xor          edx, edx
00007FF69345F8AE  call         HeapFree (00007FF693553B98h)
00007FF69345F8B3  jmp          static void Fabled_Engine::main+A50h (00007FF69345F860h)
00007FF69345F8B5  shl          qword ptr [rsp+1C0h], 6h
00007FF69345F8BE  je           static void Fabled_Engine::main+AC6h (00007FF69345F8D6h)
00007FF69345F8C0  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F8C7  xor          edx, edx
00007FF69345F8C9  mov          r8, qword ptr [rsp+C8h]
00007FF69345F8D1  call         HeapFree (00007FF693553B98h)
00007FF69345F8D6  mov          rax, qword ptr [rsp+F0h]
00007FF69345F8DE  test         rax, rax
00007FF69345F8E1  mov          rbx, qword ptr [rsp+F8h]
00007FF69345F8E9  je           static void Fabled_Engine::main+B08h (00007FF69345F918h)
00007FF69345F8EB  imul         rsi, rax, A8h
00007FF69345F8F2  mov          rcx, qword ptr [rsp+C0h]
00007FF69345F8FA  nop          word ptr [rax+rax*1], ax
00007FF69345F900  lea          rdi, [rcx+A8h]
00007FF69345F907  call         static void core::ptr::drop_in_place<naga::Function> (00007FF69345E4A0h)
00007FF69345F90C  mov          rcx, rdi
00007FF69345F90F  add          rsi, FFFFFFFFFFFFFF58h
00007FF69345F916  jne          static void Fabled_Engine::main+AF0h (00007FF69345F900h)
00007FF69345F918  mov          rax, qword ptr [rsp+100h]
00007FF69345F920  test         rax, rax
00007FF69345F923  je           static void Fabled_Engine::main+B37h (00007FF69345F947h)
00007FF69345F925  imul         rax, rax, A8h
00007FF69345F92C  test         rax, rax
00007FF69345F92F  je           static void Fabled_Engine::main+B37h (00007FF69345F947h)
00007FF69345F931  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F938  xor          edx, edx
00007FF69345F93A  mov          r8, qword ptr [rsp+C0h]
00007FF69345F942  call         HeapFree (00007FF693553B98h)
00007FF69345F947  test         rbx, rbx
00007FF69345F94A  mov          rdi, qword ptr [rsp+E0h]
00007FF69345F952  je           static void Fabled_Engine::main+B93h (00007FF69345F9A3h)
00007FF69345F954  lea          rsi, [rdi+18h]
00007FF69345F958  imul         rbx, rbx, D0h
00007FF69345F95F  jmp          static void Fabled_Engine::main+B78h (00007FF69345F988h)
00007FF69345F961  nop          word ptr cs:[rax+rax*1], ax
00007FF69345F96B  nop          dword ptr [rax+rax*1], eax
00007FF69345F970  mov          rcx, rsi
00007FF69345F973  call         static void core::ptr::drop_in_place<naga::Function> (00007FF69345E4A0h)
00007FF69345F978  add          rsi, D0h
00007FF69345F97F  add          rbx, FFFFFFFFFFFFFF30h
00007FF69345F986  je           static void Fabled_Engine::main+B93h (00007FF69345F9A3h)
00007FF69345F988  cmp          qword ptr [rsi-10h], 0h
00007FF69345F98D  je           static void Fabled_Engine::main+B60h (00007FF69345F970h)
00007FF69345F98F  mov          r8, qword ptr [rsi-18h]
00007FF69345F993  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F99A  xor          edx, edx
00007FF69345F99C  call         HeapFree (00007FF693553B98h)
00007FF69345F9A1  jmp          static void Fabled_Engine::main+B60h (00007FF69345F970h)
00007FF69345F9A3  mov          rax, qword ptr [rsp+108h]
00007FF69345F9AB  test         rax, rax
00007FF69345F9AE  je           static void Fabled_Engine::main+BBDh (00007FF69345F9CDh)
00007FF69345F9B0  imul         rax, rax, D0h
00007FF69345F9B7  test         rax, rax
00007FF69345F9BA  je           static void Fabled_Engine::main+BBDh (00007FF69345F9CDh)
00007FF69345F9BC  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F9C3  xor          edx, edx
00007FF69345F9C5  mov          r8, rdi
00007FF69345F9C8  call         HeapFree (00007FF693553B98h)
00007FF69345F9CD  test         r14, r14
00007FF69345F9D0  je           static void Fabled_Engine::main+BD3h (00007FF69345F9E3h)
00007FF69345F9D2  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345F9D9  xor          edx, edx
00007FF69345F9DB  mov          r8, r15
00007FF69345F9DE  call         HeapFree (00007FF693553B98h)

// get_binding
00007FF69345F9E3  lea          rcx, [rsp+30h]
00007FF69345F9E8  lea          rdx, [rsp+200h]
00007FF69345F9F0  xor          r8d, r8d
00007FF69345F9F3  call         static struct alloc::vec::Vec<tuple<fabled_render::material::material_graph::material_key::MaterialKey, fabled_render::material::material_graph::material_node::MaterialNode>, alloc::alloc::Global> fabled_render::shader::parser::material_parser::MaterialParser::get_binding (00007FF69346E8E0h)
00007FF69345F9F8  mov          rax, qword ptr [rsp+38h]
00007FF69345F9FD  test         rax, rax
00007FF69345FA00  je           static void Fabled_Engine::main+C12h (00007FF69345FA22h)
00007FF69345FA02  shl          rax, 5h
00007FF69345FA06  lea          rax, [rax+rax*2]
00007FF69345FA0A  test         rax, rax
00007FF69345FA0D  je           static void Fabled_Engine::main+C12h (00007FF69345FA22h)
00007FF69345FA0F  mov          r8, qword ptr [rsp+30h]
00007FF69345FA14  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345FA1B  xor          edx, edx
00007FF69345FA1D  call         HeapFree (00007FF693553B98h)

// get_group
00007FF69345FA22  lea          rcx, [rsp+30h]
00007FF69345FA27  lea          rdx, [rsp+200h]
00007FF69345FA2F  xor          r8d, r8d
00007FF69345FA32  call         static struct alloc::vec::Vec<tuple<fabled_render::material::material_graph::material_key::MaterialKey, fabled_render::material::material_graph::material_node::MaterialNode>, alloc::alloc::Global> fabled_render::shader::parser::material_parser::MaterialParser::get_group (00007FF69346E750h)
00007FF69345FA37  mov          rax, qword ptr [rsp+38h]
00007FF69345FA3C  test         rax, rax
00007FF69345FA3F  je           static void Fabled_Engine::main+C51h (00007FF69345FA61h)
00007FF69345FA41  shl          rax, 5h
00007FF69345FA45  lea          rax, [rax+rax*2]
00007FF69345FA49  test         rax, rax
00007FF69345FA4C  je           static void Fabled_Engine::main+C51h (00007FF69345FA61h)
00007FF69345FA4E  mov          r8, qword ptr [rsp+30h]
00007FF69345FA53  mov          rcx, qword ptr [00007FF6935801D8h]
00007FF69345FA5A  xor          edx, edx
00007FF69345FA5C  call         HeapFree (00007FF693553B98h)
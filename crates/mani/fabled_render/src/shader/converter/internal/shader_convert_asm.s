# --------------- Shader Convert Dissassembly -------------------
// VERT to WGSL
00007FF7CA2D2DC7  mov          rcx, rsi
00007FF7CA2D2DCA  mov          rdx, r13
00007FF7CA2D2DCD  mov          r8, r15
00007FF7CA2D2DD0  call         static void std::path::Path::components (00007FF7CA3CE110h)
00007FF7CA2D2DD5  lea          rcx, [rsp+3A0h]
00007FF7CA2D2DDD  mov          rdx, rsi
00007FF7CA2D2DE0  call         static void std::path::{{impl}}::next_back (00007FF7CA3D4400h)
00007FF7CA2D2DE5  mov          rax, qword ptr [rsp+3A0h]
00007FF7CA2D2DED  cmp          rax, 5h
00007FF7CA2D2DF1  je           static void Fabled_Engine::main+33D1h (00007FF7CA2D6051h)
00007FF7CA2D2DF7  cmp          eax, 4h
00007FF7CA2D2DFA  jne          static void Fabled_Engine::main+33D1h (00007FF7CA2D6051h)
00007FF7CA2D2E00  mov          rbp, qword ptr [rsp+3A8h]
00007FF7CA2D2E08  mov          rdx, qword ptr [rsp+3B0h]
00007FF7CA2D2E10  cmp          rdx, 2h
00007FF7CA2D2E14  jne          static void Fabled_Engine::main+1A5h (00007FF7CA2D2E25h)
00007FF7CA2D2E16  movzx        eax, word ptr [rbp]
00007FF7CA2D2E1A  cmp          eax, 2E2Eh
00007FF7CA2D2E1F  je           static void Fabled_Engine::main+33D1h (00007FF7CA2D6051h)
00007FF7CA2D2E25  lea          rcx, [rbp-1h]
00007FF7CA2D2E29  not          rbp
00007FF7CA2D2E2C  xor          eax, eax
00007FF7CA2D2E2E  nop
00007FF7CA2D2E30  cmp          rdx, rax
00007FF7CA2D2E33  je           static void Fabled_Engine::main+33D1h (00007FF7CA2D6051h)
00007FF7CA2D2E39  mov          rsi, rax
00007FF7CA2D2E3C  add          rbp, 1h
00007FF7CA2D2E40  cmp          byte ptr [rcx+rdx*1], 2Eh
00007FF7CA2D2E44  lea          rcx, [rcx-1h]
00007FF7CA2D2E48  lea          rax, [rax+1h]
00007FF7CA2D2E4C  jne          static void Fabled_Engine::main+1B0h (00007FF7CA2D2E30h)
00007FF7CA2D2E4E  mov          rcx, rdx
00007FF7CA2D2E51  sub          rcx, rax
00007FF7CA2D2E54  add          rcx, 1h
00007FF7CA2D2E58  cmp          rdx, rcx
00007FF7CA2D2E5B  jb           static void Fabled_Engine::main+35A2h (00007FF7CA2D6222h)
00007FF7CA2D2E61  mov          rcx, rdx
00007FF7CA2D2E64  sub          rcx, rbp
00007FF7CA2D2E67  xor          ebx, ebx
00007FF7CA2D2E69  cmp          rdx, rax
00007FF7CA2D2E6C  cmovne       rbx, rcx
00007FF7CA2D2E70  je           static void Fabled_Engine::main+33D1h (00007FF7CA2D6051h)
00007FF7CA2D2E76  test         rsi, rsi
00007FF7CA2D2E79  je           static void Fabled_Engine::main+379h (00007FF7CA2D2FF9h)
00007FF7CA2D2E7F  lea          rcx, [rbx+rsi*1]
00007FF7CA2D2E83  mov          rdx, rbx
00007FF7CA2D2E86  jmp          static void Fabled_Engine::main+218h (00007FF7CA2D2E98h)
00007FF7CA2D2E88  nop          dword ptr [rax+rax*1], eax
00007FF7CA2D2E90  mov          rdx, rbp
00007FF7CA2D2E93  cmp          rdx, rcx
00007FF7CA2D2E96  je           static void Fabled_Engine::main+27Eh (00007FF7CA2D2EFEh)
00007FF7CA2D2E98  lea          rbp, [rdx+1h]
00007FF7CA2D2E9C  movzx        eax, byte ptr [rdx]
00007FF7CA2D2E9F  test         al, al
00007FF7CA2D2EA1  jns          static void Fabled_Engine::main+210h (00007FF7CA2D2E90h)
00007FF7CA2D2EA3  add          rdx, 2h
00007FF7CA2D2EA7  cmp          al, E0h
00007FF7CA2D2EA9  jae          static void Fabled_Engine::main+234h (00007FF7CA2D2EB4h)
00007FF7CA2D2EAB  cmp          rbp, rcx
00007FF7CA2D2EAE  cmove        rdx, rbp
00007FF7CA2D2EB2  jmp          static void Fabled_Engine::main+213h (00007FF7CA2D2E93h)
00007FF7CA2D2EB4  cmp          al, EDh
00007FF7CA2D2EB6  jne          static void Fabled_Engine::main+258h (00007FF7CA2D2ED8h)
00007FF7CA2D2EB8  cmp          rbp, rcx
00007FF7CA2D2EBB  cmove        rdx, rbp
00007FF7CA2D2EBF  cmp          rdx, rcx
00007FF7CA2D2EC2  je           static void Fabled_Engine::main+27Eh (00007FF7CA2D2EFEh)
00007FF7CA2D2EC4  add          rdx, 1h
00007FF7CA2D2EC8  cmp          rbp, rcx
00007FF7CA2D2ECB  je           static void Fabled_Engine::main+213h (00007FF7CA2D2E93h)
00007FF7CA2D2ECD  cmp          byte ptr [rbp], A0h
00007FF7CA2D2ED1  jb           static void Fabled_Engine::main+213h (00007FF7CA2D2E93h)
00007FF7CA2D2ED3  jmp          static void Fabled_Engine::main+3588h (00007FF7CA2D6208h)
00007FF7CA2D2ED8  cmp          rbp, rcx
00007FF7CA2D2EDB  mov          rdi, rdx
00007FF7CA2D2EDE  cmove        rdi, rbp
00007FF7CA2D2EE2  lea          rdx, [rdi+1h]
00007FF7CA2D2EE6  cmp          rdi, rcx
00007FF7CA2D2EE9  cmove        rdx, rdi
00007FF7CA2D2EED  cmp          al, F0h
00007FF7CA2D2EEF  jb           static void Fabled_Engine::main+213h (00007FF7CA2D2E93h)
00007FF7CA2D2EF1  lea          rax, [rdx+1h]
00007FF7CA2D2EF5  cmp          rdx, rcx
00007FF7CA2D2EF8  cmovne       rdx, rax
00007FF7CA2D2EFC  jmp          static void Fabled_Engine::main+213h (00007FF7CA2D2E93h)
00007FF7CA2D2EFE  cmp          rsi, 3h
00007FF7CA2D2F02  je           static void Fabled_Engine::main+361h (00007FF7CA2D2FE1h)
00007FF7CA2D2F08  cmp          rsi, 4h
00007FF7CA2D2F0C  jne          static void Fabled_Engine::main+379h (00007FF7CA2D2FF9h)
00007FF7CA2D2F12  cmp          dword ptr [rbx], 6C736777h
00007FF7CA2D2F18  je           static void Fabled_Engine::main+77Bh (00007FF7CA2D33FBh)
00007FF7CA2D2F1E  cmp          dword ptr [rbx], 74726576h
00007FF7CA2D2F24  je           static void Fabled_Engine::main+2BAh (00007FF7CA2D2F3Ah)
00007FF7CA2D2F26  cmp          dword ptr [rbx], 67617266h
00007FF7CA2D2F2C  je           static void Fabled_Engine::main+2BAh (00007FF7CA2D2F3Ah)
00007FF7CA2D2F2E  cmp          dword ptr [rbx], 706D6F63h
00007FF7CA2D2F34  jne          static void Fabled_Engine::main+379h (00007FF7CA2D2FF9h)
00007FF7CA2D2F3A  mov          qword ptr [rsp+30h], r12
00007FF7CA2D2F3F  lea          rcx, [rsp+600h]
00007FF7CA2D2F47  mov          rdx, r13
00007FF7CA2D2F4A  mov          r8, r15
00007FF7CA2D2F4D  call         static void std::fs::read_to_string::inner (00007FF7CA3D3A00h)
00007FF7CA2D2F52  cmp          dword ptr [rsp+600h], 1h
00007FF7CA2D2F5A  mov          rdi, qword ptr [rsp+610h]
00007FF7CA2D2F62  mov          rsi, qword ptr [rsp+608h]
00007FF7CA2D2F6A  mov          qword ptr [rsp+38h], r13
00007FF7CA2D2F6F  je           static void Fabled_Engine::main+38E3h (00007FF7CA2D6563h)
00007FF7CA2D2F75  mov          r12, qword ptr [rsp+618h]
00007FF7CA2D2F7D  pxor         xmm0, xmm0
00007FF7CA2D2F81  movdqu       xmmword ptr [rsp+180h], xmm0
00007FF7CA2D2F8A  mov          qword ptr [rsp+170h], 0h
00007FF7CA2D2F96  lea          rax, [00007FF7CA3ECA20h]
00007FF7CA2D2F9D  mov          qword ptr [rsp+178h], rax
00007FF7CA2D2FA5  cmp          dword ptr [rbx], 74726576h
00007FF7CA2D2FAB  je           static void Fabled_Engine::main+A58h (00007FF7CA2D36D8h)
00007FF7CA2D2FB1  cmp          dword ptr [rbx], 67617266h
00007FF7CA2D2FB7  je           static void Fabled_Engine::main+A69h (00007FF7CA2D36E9h)
00007FF7CA2D2FBD  mov          r14b, 2h
00007FF7CA2D2FC0  cmp          dword ptr [rbx], 706D6F63h
00007FF7CA2D2FC6  jne          static void Fabled_Engine::main+3B97h (00007FF7CA2D6817h)
00007FF7CA2D2FCC  mov          rcx, qword ptr [00007FF7CA4071D8h]
00007FF7CA2D2FD3  test         rcx, rcx
00007FF7CA2D2FD6  jne          static void Fabled_Engine::main+A90h (00007FF7CA2D3710h)
00007FF7CA2D2FDC  jmp          static void Fabled_Engine::main+A78h (00007FF7CA2D36F8h)
00007FF7CA2D2FE1  movzx        eax, word ptr [rbx]
00007FF7CA2D2FE4  xor          eax, 7073h
00007FF7CA2D2FE9  movzx        ecx, byte ptr [rbx+2h]
00007FF7CA2D2FED  xor          ecx, 76h
00007FF7CA2D2FF0  or           cx, ax
00007FF7CA2D2FF3  je           static void Fabled_Engine::main+2906h (00007FF7CA2D5586h)
00007FF7CA2D2FF9  mov          rax, qword ptr [00007FF7CA3FF888h]
00007FF7CA2D3000  pxor         xmm0, xmm0
00007FF7CA2D3004  movdqu       xmmword ptr [rsp+1A8h], xmm0
00007FF7CA2D300D  movdqa       xmmword ptr [rsp+1C0h], xmm0
00007FF7CA2D3016  movdqu       xmmword ptr [rsp+1D8h], xmm0
00007FF7CA2D301F  movdqa       xmmword ptr [rsp+1F0h], xmm0
00007FF7CA2D3028  movdqu       xmmword ptr [rsp+208h], xmm0
00007FF7CA2D3031  mov          qword ptr [rsp+1A0h], rax
00007FF7CA2D3039  mov          qword ptr [rsp+1B8h], rax
00007FF7CA2D3041  mov          qword ptr [rsp+1D0h], rax
00007FF7CA2D3049  mov          qword ptr [rsp+1E8h], rax
00007FF7CA2D3051  mov          qword ptr [rsp+200h], rax
00007FF7CA2D3059  lea          rcx, [rsp+600h]
00007FF7CA2D3061  lea          rdx, [rsp+1A0h]
00007FF7CA2D3069  call         static union enum$<core::result::Result<naga::valid::ModuleInfo, anyhow::Error>> fabled_render::shader::shader_validator::{{impl}}::validate (00007FF7CA2E5550h)
00007FF7CA2D306E  cmp          dword ptr [rsp+600h], 1h
00007FF7CA2D3076  mov          rbx, qword ptr [rsp+608h]
00007FF7CA2D307E  je           static void Fabled_Engine::main+35B0h (00007FF7CA2D6230h)
00007FF7CA2D3084  mov          rax, qword ptr [rsp+630h]
00007FF7CA2D308C  mov          qword ptr [rsp+3C8h], rax
00007FF7CA2D3094  movups       xmm0, xmmword ptr [rsp+610h]
00007FF7CA2D309C  movups       xmm1, xmmword ptr [rsp+620h]
00007FF7CA2D30A4  movups       xmmword ptr [rsp+3B8h], xmm1
00007FF7CA2D30AC  movups       xmmword ptr [rsp+3A8h], xmm0
00007FF7CA2D30B4  mov          qword ptr [rsp+3A0h], rbx
00007FF7CA2D30BC  lea          rcx, [rsp+3A0h]
00007FF7CA2D30C4  call         static void core::ptr::drop_in_place<alloc::vec::Vec<naga::valid::analyzer::FunctionInfo, alloc::alloc::Global>> (00007FF7CA2D2170h)
00007FF7CA2D30C9  lea          rcx, [rsp+3B8h]
00007FF7CA2D30D1  call         static void core::ptr::drop_in_place<alloc::vec::Vec<naga::valid::analyzer::FunctionInfo, alloc::alloc::Global>> (00007FF7CA2D2170h)
00007FF7CA2D30D6  mov          rsi, qword ptr [rsp+1A0h]
00007FF7CA2D30DE  movups       xmm0, xmmword ptr [rsp+1A8h]
00007FF7CA2D30E6  movaps       xmmword ptr [rsp+980h], xmm0
00007FF7CA2D30EE  movups       xmm0, xmmword ptr [rsp+1B8h]
00007FF7CA2D30F6  movaps       xmmword ptr [rsp+990h], xmm0
00007FF7CA2D30FE  movups       xmm0, xmmword ptr [rsp+1C8h]
00007FF7CA2D3106  movaps       xmmword ptr [rsp+9A0h], xmm0
00007FF7CA2D310E  movups       xmm0, xmmword ptr [rsp+1D8h]
00007FF7CA2D3116  movaps       xmmword ptr [rsp+9B0h], xmm0
00007FF7CA2D311E  movups       xmm0, xmmword ptr [rsp+1E8h]
00007FF7CA2D3126  movaps       xmmword ptr [rsp+9C0h], xmm0
00007FF7CA2D312E  movups       xmm0, xmmword ptr [rsp+1F8h]
00007FF7CA2D3136  movaps       xmmword ptr [rsp+9D0h], xmm0
00007FF7CA2D313E  movups       xmm0, xmmword ptr [rsp+208h]
00007FF7CA2D3146  movaps       xmmword ptr [rsp+9E0h], xmm0
00007FF7CA2D314E  test         r12, r12
00007FF7CA2D3151  je           static void Fabled_Engine::main+4E4h (00007FF7CA2D3164h)
00007FF7CA2D3153  mov          rcx, qword ptr [00007FF7CA4071D8h]
00007FF7CA2D315A  xor          edx, edx
00007FF7CA2D315C  mov          r8, r13
00007FF7CA2D315F  call         HeapFree (00007FF7CA3D8F0Ch)
00007FF7CA2D3164  movaps       xmm0, xmmword ptr [rsp+9E0h]
00007FF7CA2D316C  movups       xmmword ptr [rsp+AD0h], xmm0
00007FF7CA2D3174  movaps       xmm0, xmmword ptr [rsp+9D0h]
00007FF7CA2D317C  movups       xmmword ptr [rsp+AC0h], xmm0
00007FF7CA2D3184  movaps       xmm0, xmmword ptr [rsp+9C0h]
00007FF7CA2D318C  movups       xmmword ptr [rsp+AB0h], xmm0
00007FF7CA2D3194  movdqa       xmm0, xmmword ptr [rsp+980h]
00007FF7CA2D319D  movaps       xmm1, xmmword ptr [rsp+990h]
00007FF7CA2D31A5  movaps       xmm2, xmmword ptr [rsp+9A0h]
00007FF7CA2D31AD  movdqa       xmm3, xmmword ptr [rsp+9B0h]
00007FF7CA2D31B6  movdqu       xmmword ptr [rsp+AA0h], xmm3
00007FF7CA2D31BF  movups       xmmword ptr [rsp+A90h], xmm2
00007FF7CA2D31C7  movups       xmmword ptr [rsp+A80h], xmm1
00007FF7CA2D31CF  movdqu       xmmword ptr [rsp+A70h], xmm0
00007FF7CA2D31D8  mov          qword ptr [rsp+A68h], rsi
00007FF7CA2D31E0  lea          rcx, [rsp+600h]
00007FF7CA2D31E8  lea          rdx, [rsp+A68h]
00007FF7CA2D31F0  call         static union enum$<core::result::Result<naga::valid::ModuleInfo, anyhow::Error>> fabled_render::shader::shader_validator::{{impl}}::validate (00007FF7CA2E5550h)
00007FF7CA2D31F5  cmp          dword ptr [rsp+600h], 1h
00007FF7CA2D31FD  mov          rbx, qword ptr [rsp+608h]
00007FF7CA2D3205  je           static void Fabled_Engine::main+3806h (00007FF7CA2D6486h)
00007FF7CA2D320B  mov          rax, qword ptr [rsp+630h]
00007FF7CA2D3213  mov          qword ptr [rsp+1C8h], rax
00007FF7CA2D321B  movups       xmm0, xmmword ptr [rsp+610h]
00007FF7CA2D3223  movups       xmm1, xmmword ptr [rsp+620h]
00007FF7CA2D322B  movups       xmmword ptr [rsp+1B8h], xmm1
00007FF7CA2D3233  movups       xmmword ptr [rsp+1A8h], xmm0
00007FF7CA2D323B  mov          qword ptr [rsp+1A0h], rbx
00007FF7CA2D3243  mov          rax, qword ptr [00007FF7CA3FF9E0h]
00007FF7CA2D324A  xorps        xmm0, xmm0
00007FF7CA2D324D  movups       xmmword ptr [rsp+3A8h], xmm0
00007FF7CA2D3255  mov          qword ptr [rsp+3A0h], rax
00007FF7CA2D325D  movups       xmmword ptr [rsp+3C8h], xmm0
00007FF7CA2D3265  mov          qword ptr [rsp+3B8h], 0h
00007FF7CA2D3271  lea          rax, [00007FF7CA3ECA20h]
00007FF7CA2D3278  mov          qword ptr [rsp+3C0h], rax
00007FF7CA2D3280  movups       xmmword ptr [rsp+3E8h], xmm0
00007FF7CA2D3288  movups       xmmword ptr [rsp+408h], xmm0
00007FF7CA2D3290  mov          rcx, qword ptr [00007FF7CA3FF888h]
00007FF7CA2D3297  movups       xmmword ptr [rsp+420h], xmm0
00007FF7CA2D329F  mov          dword ptr [rsp+430h], 0h
00007FF7CA2D32AA  movups       xmmword ptr [rsp+448h], xmm0
00007FF7CA2D32B2  mov          qword ptr [rsp+438h], 0h
00007FF7CA2D32BE  mov          qword ptr [rsp+440h], rax
00007FF7CA2D32C6  mov          rdx, qword ptr [00007FF7CA3F9E48h]
00007FF7CA2D32CD  movups       xmmword ptr [rsp+460h], xmm0
00007FF7CA2D32D5  mov          qword ptr [rsp+3D8h], 0h
00007FF7CA2D32E1  mov          qword ptr [rsp+3E0h], rax
00007FF7CA2D32E9  mov          qword ptr [rsp+3F8h], 0h
00007FF7CA2D32F5  mov          qword ptr [rsp+400h], rax
00007FF7CA2D32FD  mov          qword ptr [rsp+418h], rcx
00007FF7CA2D3305  mov          qword ptr [rsp+458h], rdx
00007FF7CA2D330D  lea          rcx, [rsp+600h]
00007FF7CA2D3315  lea          rdx, [rsp+3A0h]
00007FF7CA2D331D  lea          r8, [rsp+A68h]
00007FF7CA2D3325  lea          r9, [rsp+1A0h]
00007FF7CA2D332D  call         static union enum$<core::result::Result<tuple<>, enum$<naga::back::wgsl::Error>>, 0, 3, Err> naga::back::wgsl::writer::Writer<alloc::string::String>::write<alloc::string::String> (00007FF7CA359420h)
00007FF7CA2D3332  mov          al, byte ptr [rsp+600h]
00007FF7CA2D3339  cmp          al, 4h
00007FF7CA2D333B  jne          static void Fabled_Engine::main+35CCh (00007FF7CA2D624Ch)
00007FF7CA2D3341  lea          rsi, [rsp+600h]
00007FF7CA2D3349  lea          rdx, [rsp+3A0h]
00007FF7CA2D3351  mov          r8d, D0h
00007FF7CA2D3357  mov          rcx, rsi
00007FF7CA2D335A  call         memcpy (00007FF7CA3D9FF7h)
00007FF7CA2D335F  lea          rcx, [rsp+228h]
00007FF7CA2D3367  mov          rdx, rsi
00007FF7CA2D336A  call         static struct alloc::string::String naga::back::wgsl::writer::Writer<alloc::string::String>::finish<alloc::string::String> (00007FF7CA365A20h)
00007FF7CA2D336F  mov          rsi, qword ptr [rsp+228h]
00007FF7CA2D3377  mov          rdi, qword ptr [rsp+230h]
00007FF7CA2D337F  lea          rcx, [rsp+1A0h]
00007FF7CA2D3387  call         static void core::ptr::drop_in_place<alloc::vec::Vec<naga::valid::analyzer::FunctionInfo, alloc::alloc::Global>> (00007FF7CA2D2170h)
00007FF7CA2D338C  lea          rcx, [rsp+1B8h]
00007FF7CA2D3394  call         static void core::ptr::drop_in_place<alloc::vec::Vec<naga::valid::analyzer::FunctionInfo, alloc::alloc::Global>> (00007FF7CA2D2170h)
00007FF7CA2D3399  lea          rcx, [rsp+A68h]
00007FF7CA2D33A1  call         static void core::ptr::drop_in_place<naga::Module> (00007FF7CA2D1160h)
00007FF7CA2D33A6  lea          rcx, [rsp+600h]


// VERT to Spir_v
00007FF6412B2DE7  mov          rcx, rsi
00007FF6412B2DEA  mov          rdx, r13
00007FF6412B2DED  mov          r8, r15
00007FF6412B2DF0  call         static void std::path::Path::components (00007FF6413B6BC0h)
00007FF6412B2DF5  lea          rcx, [rsp+800h]
00007FF6412B2DFD  mov          rdx, rsi
00007FF6412B2E00  call         static void std::path::{{impl}}::next_back (00007FF6413BCEB0h)
00007FF6412B2E05  mov          rax, qword ptr [rsp+800h]
00007FF6412B2E0D  cmp          rax, 5h
00007FF6412B2E11  je           static void Fabled_Engine::main+38B1h (00007FF6412B6551h)
00007FF6412B2E17  cmp          eax, 4h
00007FF6412B2E1A  jne          static void Fabled_Engine::main+38B1h (00007FF6412B6551h)
00007FF6412B2E20  mov          rbp, qword ptr [rsp+808h]
00007FF6412B2E28  mov          rdx, qword ptr [rsp+810h]
00007FF6412B2E30  cmp          rdx, 2h
00007FF6412B2E34  jne          static void Fabled_Engine::main+1A5h (00007FF6412B2E45h)
00007FF6412B2E36  movzx        eax, word ptr [rbp]
00007FF6412B2E3A  cmp          eax, 2E2Eh
00007FF6412B2E3F  je           static void Fabled_Engine::main+38B1h (00007FF6412B6551h)
00007FF6412B2E45  lea          rcx, [rbp-1h]
00007FF6412B2E49  not          rbp
00007FF6412B2E4C  xor          eax, eax
00007FF6412B2E4E  nop
00007FF6412B2E50  cmp          rdx, rax
00007FF6412B2E53  je           static void Fabled_Engine::main+38B1h (00007FF6412B6551h)
00007FF6412B2E59  mov          rsi, rax
00007FF6412B2E5C  add          rbp, 1h
00007FF6412B2E60  cmp          byte ptr [rcx+rdx*1], 2Eh
00007FF6412B2E64  lea          rcx, [rcx-1h]
00007FF6412B2E68  lea          rax, [rax+1h]
00007FF6412B2E6C  jne          static void Fabled_Engine::main+1B0h (00007FF6412B2E50h)
00007FF6412B2E6E  mov          rcx, rdx
00007FF6412B2E71  sub          rcx, rax
00007FF6412B2E74  add          rcx, 1h
00007FF6412B2E78  cmp          rdx, rcx
00007FF6412B2E7B  jb           static void Fabled_Engine::main+3A82h (00007FF6412B6722h)
00007FF6412B2E81  mov          rcx, rdx
00007FF6412B2E84  sub          rcx, rbp
00007FF6412B2E87  xor          ebx, ebx
00007FF6412B2E89  cmp          rdx, rax
00007FF6412B2E8C  cmovne       rbx, rcx
00007FF6412B2E90  je           static void Fabled_Engine::main+38B1h (00007FF6412B6551h)
00007FF6412B2E96  test         rsi, rsi
00007FF6412B2E99  je           static void Fabled_Engine::main+379h (00007FF6412B3019h)
00007FF6412B2E9F  lea          rcx, [rbx+rsi*1]
00007FF6412B2EA3  mov          rdx, rbx
00007FF6412B2EA6  jmp          static void Fabled_Engine::main+218h (00007FF6412B2EB8h)
00007FF6412B2EA8  nop          dword ptr [rax+rax*1], eax
00007FF6412B2EB0  mov          rdx, rbp
00007FF6412B2EB3  cmp          rdx, rcx
00007FF6412B2EB6  je           static void Fabled_Engine::main+27Eh (00007FF6412B2F1Eh)
00007FF6412B2EB8  lea          rbp, [rdx+1h]
00007FF6412B2EBC  movzx        eax, byte ptr [rdx]
00007FF6412B2EBF  test         al, al
00007FF6412B2EC1  jns          static void Fabled_Engine::main+210h (00007FF6412B2EB0h)
00007FF6412B2EC3  add          rdx, 2h
00007FF6412B2EC7  cmp          al, E0h
00007FF6412B2EC9  jae          static void Fabled_Engine::main+234h (00007FF6412B2ED4h)
00007FF6412B2ECB  cmp          rbp, rcx
00007FF6412B2ECE  cmove        rdx, rbp
00007FF6412B2ED2  jmp          static void Fabled_Engine::main+213h (00007FF6412B2EB3h)
00007FF6412B2ED4  cmp          al, EDh
00007FF6412B2ED6  jne          static void Fabled_Engine::main+258h (00007FF6412B2EF8h)
00007FF6412B2ED8  cmp          rbp, rcx
00007FF6412B2EDB  cmove        rdx, rbp
00007FF6412B2EDF  cmp          rdx, rcx
00007FF6412B2EE2  je           static void Fabled_Engine::main+27Eh (00007FF6412B2F1Eh)
00007FF6412B2EE4  add          rdx, 1h
00007FF6412B2EE8  cmp          rbp, rcx
00007FF6412B2EEB  je           static void Fabled_Engine::main+213h (00007FF6412B2EB3h)
00007FF6412B2EED  cmp          byte ptr [rbp], A0h
00007FF6412B2EF1  jb           static void Fabled_Engine::main+213h (00007FF6412B2EB3h)
00007FF6412B2EF3  jmp          static void Fabled_Engine::main+3A68h (00007FF6412B6708h)
00007FF6412B2EF8  cmp          rbp, rcx
00007FF6412B2EFB  mov          rdi, rdx
00007FF6412B2EFE  cmove        rdi, rbp
00007FF6412B2F02  lea          rdx, [rdi+1h]
00007FF6412B2F06  cmp          rdi, rcx
00007FF6412B2F09  cmove        rdx, rdi
00007FF6412B2F0D  cmp          al, F0h
00007FF6412B2F0F  jb           static void Fabled_Engine::main+213h (00007FF6412B2EB3h)
00007FF6412B2F11  lea          rax, [rdx+1h]
00007FF6412B2F15  cmp          rdx, rcx
00007FF6412B2F18  cmovne       rdx, rax
00007FF6412B2F1C  jmp          static void Fabled_Engine::main+213h (00007FF6412B2EB3h)
00007FF6412B2F1E  cmp          rsi, 3h
00007FF6412B2F22  je           static void Fabled_Engine::main+361h (00007FF6412B3001h)
00007FF6412B2F28  cmp          rsi, 4h
00007FF6412B2F2C  jne          static void Fabled_Engine::main+379h (00007FF6412B3019h)
00007FF6412B2F32  cmp          dword ptr [rbx], 6C736777h
00007FF6412B2F38  je           static void Fabled_Engine::main+C62h (00007FF6412B3902h)
00007FF6412B2F3E  cmp          dword ptr [rbx], 74726576h
00007FF6412B2F44  je           static void Fabled_Engine::main+2BAh (00007FF6412B2F5Ah)
00007FF6412B2F46  cmp          dword ptr [rbx], 67617266h
00007FF6412B2F4C  je           static void Fabled_Engine::main+2BAh (00007FF6412B2F5Ah)
00007FF6412B2F4E  cmp          dword ptr [rbx], 706D6F63h
00007FF6412B2F54  jne          static void Fabled_Engine::main+379h (00007FF6412B3019h)
00007FF6412B2F5A  mov          qword ptr [rsp+28h], r12
00007FF6412B2F5F  lea          rcx, [rsp+3D0h]
00007FF6412B2F67  mov          rdx, r13
00007FF6412B2F6A  mov          r8, r15
00007FF6412B2F6D  call         static void std::fs::read_to_string::inner (00007FF6413BC4B0h)
00007FF6412B2F72  cmp          dword ptr [rsp+3D0h], 1h
00007FF6412B2F7A  mov          rdi, qword ptr [rsp+3E0h]
00007FF6412B2F82  mov          rsi, qword ptr [rsp+3D8h]
00007FF6412B2F8A  mov          qword ptr [rsp+38h], r13
00007FF6412B2F8F  je           static void Fabled_Engine::main+3D13h (00007FF6412B69B3h)
00007FF6412B2F95  mov          r12, qword ptr [rsp+3E8h]
00007FF6412B2F9D  pxor         xmm0, xmm0
00007FF6412B2FA1  movdqu       xmmword ptr [rsp+180h], xmm0
00007FF6412B2FAA  mov          qword ptr [rsp+170h], 0h
00007FF6412B2FB6  lea          rax, [00007FF6413D5820h]
00007FF6412B2FBD  mov          qword ptr [rsp+178h], rax
00007FF6412B2FC5  cmp          dword ptr [rbx], 74726576h
00007FF6412B2FCB  je           static void Fabled_Engine::main+F38h (00007FF6412B3BD8h)
00007FF6412B2FD1  cmp          dword ptr [rbx], 67617266h
00007FF6412B2FD7  je           static void Fabled_Engine::main+F49h (00007FF6412B3BE9h)
00007FF6412B2FDD  mov          r14b, 2h
00007FF6412B2FE0  cmp          dword ptr [rbx], 706D6F63h
00007FF6412B2FE6  jne          static void Fabled_Engine::main+3FC7h (00007FF6412B6C67h)
00007FF6412B2FEC  mov          rcx, qword ptr [00007FF6413EF1D8h]
00007FF6412B2FF3  test         rcx, rcx
00007FF6412B2FF6  jne          static void Fabled_Engine::main+F70h (00007FF6412B3C10h)
00007FF6412B2FFC  jmp          static void Fabled_Engine::main+F58h (00007FF6412B3BF8h)
00007FF6412B3001  movzx        eax, word ptr [rbx]
00007FF6412B3004  xor          eax, 7073h
00007FF6412B3009  movzx        ecx, byte ptr [rbx+2h]
00007FF6412B300D  xor          ecx, 76h
00007FF6412B3010  or           cx, ax
00007FF6412B3013  je           static void Fabled_Engine::main+2DE6h (00007FF6412B5A86h)
00007FF6412B3019  mov          rax, qword ptr [00007FF6413E7C60h]
00007FF6412B3020  pxor         xmm0, xmm0
00007FF6412B3024  movdqu       xmmword ptr [rsp+2D8h], xmm0
00007FF6412B302D  movdqa       xmmword ptr [rsp+2F0h], xmm0
00007FF6412B3036  movdqu       xmmword ptr [rsp+308h], xmm0
00007FF6412B303F  movdqa       xmmword ptr [rsp+320h], xmm0
00007FF6412B3048  movdqu       xmmword ptr [rsp+338h], xmm0
00007FF6412B3051  mov          qword ptr [rsp+2D0h], rax
00007FF6412B3059  mov          qword ptr [rsp+2E8h], rax
00007FF6412B3061  mov          qword ptr [rsp+300h], rax
00007FF6412B3069  mov          qword ptr [rsp+318h], rax
00007FF6412B3071  mov          qword ptr [rsp+330h], rax
00007FF6412B3079  lea          rcx, [rsp+3D0h]
00007FF6412B3081  lea          rdi, [rsp+2D0h]
00007FF6412B3089  mov          rdx, rdi
00007FF6412B308C  call         static union enum$<core::result::Result<naga::valid::ModuleInfo, anyhow::Error>> fabled_render::shader::shader_validator::{{impl}}::validate (00007FF6412C4A70h)
00007FF6412B3091  cmp          dword ptr [rsp+3D0h], 1h
00007FF6412B3099  mov          rbx, qword ptr [rsp+3D8h]
00007FF6412B30A1  je           static void Fabled_Engine::main+3A90h (00007FF6412B6730h)
00007FF6412B30A7  mov          rax, qword ptr [rsp+400h]
00007FF6412B30AF  mov          qword ptr [rsp+828h], rax
00007FF6412B30B7  movups       xmm0, xmmword ptr [rsp+3E0h]
00007FF6412B30BF  movups       xmm1, xmmword ptr [rsp+3F0h]
00007FF6412B30C7  movups       xmmword ptr [rsp+818h], xmm1
00007FF6412B30CF  movups       xmmword ptr [rsp+808h], xmm0
00007FF6412B30D7  mov          qword ptr [rsp+800h], rbx
00007FF6412B30DF  lea          rcx, [rsp+800h]
00007FF6412B30E7  call         static void core::ptr::drop_in_place<alloc::vec::Vec<naga::valid::analyzer::FunctionInfo, alloc::alloc::Global>> (00007FF6412B2130h)
00007FF6412B30EC  lea          rcx, [rsp+818h]
00007FF6412B30F4  call         static void core::ptr::drop_in_place<alloc::vec::Vec<naga::valid::analyzer::FunctionInfo, alloc::alloc::Global>> (00007FF6412B2130h)
00007FF6412B30F9  mov          rsi, qword ptr [rsp+2D0h]
00007FF6412B3101  movups       xmm0, xmmword ptr [rsp+2D8h]
00007FF6412B3109  movaps       xmmword ptr [rsp+9F0h], xmm0
00007FF6412B3111  movups       xmm0, xmmword ptr [rsp+2E8h]
00007FF6412B3119  movaps       xmmword ptr [rsp+A00h], xmm0
00007FF6412B3121  movups       xmm0, xmmword ptr [rsp+2F8h]
00007FF6412B3129  movaps       xmmword ptr [rsp+A10h], xmm0
00007FF6412B3131  movups       xmm0, xmmword ptr [rsp+308h]
00007FF6412B3139  movaps       xmmword ptr [rsp+A20h], xmm0
00007FF6412B3141  movups       xmm0, xmmword ptr [rsp+318h]
00007FF6412B3149  movaps       xmmword ptr [rsp+A30h], xmm0
00007FF6412B3151  movups       xmm0, xmmword ptr [rsp+328h]
00007FF6412B3159  movaps       xmmword ptr [rsp+A40h], xmm0
00007FF6412B3161  movups       xmm0, xmmword ptr [rsp+338h]
00007FF6412B3169  movaps       xmmword ptr [rsp+A50h], xmm0
00007FF6412B3171  test         r12, r12
00007FF6412B3174  je           static void Fabled_Engine::main+4E7h (00007FF6412B3187h)
00007FF6412B3176  mov          rcx, qword ptr [00007FF6413EF1D8h]
00007FF6412B317D  xor          edx, edx
00007FF6412B317F  mov          r8, r13
00007FF6412B3182  call         HeapFree (00007FF6413C19BCh)
00007FF6412B3187  movaps       xmm0, xmmword ptr [rsp+A50h]
00007FF6412B318F  movups       xmmword ptr [rsp+C30h], xmm0
00007FF6412B3197  movaps       xmm0, xmmword ptr [rsp+A40h]
00007FF6412B319F  movups       xmmword ptr [rsp+C20h], xmm0
00007FF6412B31A7  movaps       xmm0, xmmword ptr [rsp+A30h]
00007FF6412B31AF  movups       xmmword ptr [rsp+C10h], xmm0
00007FF6412B31B7  movdqa       xmm0, xmmword ptr [rsp+9F0h]
00007FF6412B31C0  movaps       xmm1, xmmword ptr [rsp+A00h]
00007FF6412B31C8  movaps       xmm2, xmmword ptr [rsp+A10h]
00007FF6412B31D0  movdqa       xmm3, xmmword ptr [rsp+A20h]
00007FF6412B31D9  movdqu       xmmword ptr [rsp+C00h], xmm3
00007FF6412B31E2  movups       xmmword ptr [rsp+BF0h], xmm2
00007FF6412B31EA  movups       xmmword ptr [rsp+BE0h], xmm1
00007FF6412B31F2  movdqu       xmmword ptr [rsp+BD0h], xmm0
00007FF6412B31FB  mov          qword ptr [rsp+BC8h], rsi
00007FF6412B3203  lea          rcx, [rsp+3D0h]
00007FF6412B320B  lea          rdx, [rsp+BC8h]
00007FF6412B3213  call         static union enum$<core::result::Result<naga::valid::ModuleInfo, anyhow::Error>> fabled_render::shader::shader_validator::{{impl}}::validate (00007FF6412C4A70h)
00007FF6412B3218  cmp          dword ptr [rsp+3D0h], 1h
00007FF6412B3220  mov          rbx, qword ptr [rsp+3D8h]
00007FF6412B3228  je           static void Fabled_Engine::main+3C36h (00007FF6412B68D6h)
00007FF6412B322E  mov          rax, qword ptr [rsp+400h]
00007FF6412B3236  mov          qword ptr [rsp+828h], rax
00007FF6412B323E  movups       xmm0, xmmword ptr [rsp+3E0h]
00007FF6412B3246  movups       xmm1, xmmword ptr [rsp+3F0h]
00007FF6412B324E  movups       xmmword ptr [rsp+818h], xmm1
00007FF6412B3256  movups       xmmword ptr [rsp+808h], xmm0
00007FF6412B325E  mov          qword ptr [rsp+800h], rbx
00007FF6412B3266  mov          rsi, qword ptr [00007FF6413E21F0h]
00007FF6412B326D  mov          qword ptr [rsp+2D0h], rsi
00007FF6412B3275  pxor         xmm6, xmm6
00007FF6412B3279  movdqu       xmmword ptr [rsp+2D8h], xmm6
00007FF6412B3282  movdqu       xmmword ptr [rsp+1B0h], xmm6
00007FF6412B328B  mov          qword ptr [rsp+1A0h], 0h
00007FF6412B3297  lea          rbx, [00007FF6413D5820h]
00007FF6412B329E  mov          qword ptr [rsp+1A8h], rbx
00007FF6412B32A6  lea          rcx, [rsp+1A0h]
00007FF6412B32AE  mov          edx, 1h
00007FF6412B32B3  call         static bool hashbrown::set::HashSet<enum$<spirv_headers::Capability>, core::hash::BuildHasherDefault<fxhash::FxHasher>, alloc::alloc::Global>::insert<enum$<spirv_headers::Capability>,core::hash::BuildHasherDefault<fxhash::FxHasher>,alloc::alloc::Global> (00007FF6412DCC30h)
00007FF6412B32B8  movups       xmm0, xmmword ptr [rsp+1A0h]
00007FF6412B32C0  movups       xmm1, xmmword ptr [rsp+1B0h]
00007FF6412B32C8  mov          rax, qword ptr [00007FF6413E7C60h]
00007FF6412B32CF  movdqa       xmmword ptr [rsp+3B0h], xmm6
00007FF6412B32D8  movdqa       xmmword ptr [rsp+9D0h], xmm6
00007FF6412B32E1  movdqa       xmmword ptr [rsp+9C0h], xmm6
00007FF6412B32EA  movdqa       xmmword ptr [rsp+9B0h], xmm6
00007FF6412B32F3  movdqa       xmmword ptr [rsp+990h], xmm6
00007FF6412B32FC  movdqa       xmmword ptr [rsp+980h], xmm6
00007FF6412B3305  movdqa       xmmword ptr [rsp+970h], xmm6
00007FF6412B330E  movdqa       xmmword ptr [rsp+BB0h], xmm6
00007FF6412B3317  movdqa       xmmword ptr [rsp+1A0h], xmm6
00007FF6412B3320  movdqa       xmmword ptr [rsp+60h], xmm6
00007FF6412B3326  movdqa       xmmword ptr [rsp+9F0h], xmm6
00007FF6412B332F  movdqa       xmmword ptr [rsp+D0h], xmm6
00007FF6412B3338  movdqa       xmmword ptr [rsp+780h], xmm6
00007FF6412B3341  movdqa       xmmword ptr [rsp+700h], xmm6
00007FF6412B334A  movdqa       xmmword ptr [rsp+350h], xmm6
00007FF6412B3353  movdqa       xmmword ptr [rsp+140h], xmm6
00007FF6412B335C  movdqa       xmmword ptr [rsp+940h], xmm6
00007FF6412B3365  movdqa       xmmword ptr [rsp+750h], xmm6
00007FF6412B336E  movdqa       xmmword ptr [rsp+390h], xmm6
00007FF6412B3377  movdqa       xmmword ptr [rsp+170h], xmm6
00007FF6412B3380  movdqa       xmmword ptr [rsp+9A0h], xmm6
00007FF6412B3389  movdqa       xmmword ptr [rsp+BA0h], xmm6
00007FF6412B3392  movdqa       xmmword ptr [rsp+B90h], xmm6
00007FF6412B339B  movdqa       xmmword ptr [rsp+B80h], xmm6
00007FF6412B33A4  movdqa       xmmword ptr [rsp+B70h], xmm6
00007FF6412B33AD  movdqa       xmmword ptr [rsp+B60h], xmm6
00007FF6412B33B6  movdqa       xmmword ptr [rsp+B50h], xmm6
00007FF6412B33BF  movdqa       xmmword ptr [rsp+B40h], xmm6
00007FF6412B33C8  movdqa       xmmword ptr [rsp+B30h], xmm6
00007FF6412B33D1  movdqa       xmmword ptr [rsp+B20h], xmm6
00007FF6412B33DA  movdqa       xmmword ptr [rsp+B10h], xmm6
00007FF6412B33E3  movdqa       xmmword ptr [rsp+B00h], xmm6
00007FF6412B33EC  movdqa       xmmword ptr [rsp+AF0h], xmm6
00007FF6412B33F5  movaps       xmm2, xmmword ptr [rsp+3B0h]
00007FF6412B33FD  movaps       xmmword ptr [rsp+AE0h], xmm2
00007FF6412B3405  movaps       xmm2, xmmword ptr [rsp+9D0h]
00007FF6412B340D  movaps       xmmword ptr [rsp+AD0h], xmm2
00007FF6412B3415  movaps       xmm2, xmmword ptr [rsp+9C0h]
00007FF6412B341D  movaps       xmmword ptr [rsp+AC0h], xmm2
00007FF6412B3425  movaps       xmm2, xmmword ptr [rsp+9B0h]
00007FF6412B342D  movaps       xmmword ptr [rsp+AB0h], xmm2
00007FF6412B3435  movaps       xmm2, xmmword ptr [rsp+9A0h]
00007FF6412B343D  movaps       xmmword ptr [rsp+AA0h], xmm2
00007FF6412B3445  movaps       xmm2, xmmword ptr [rsp+990h]
00007FF6412B344D  movaps       xmmword ptr [rsp+A90h], xmm2
00007FF6412B3455  movaps       xmm2, xmmword ptr [rsp+980h]
00007FF6412B345D  movaps       xmmword ptr [rsp+A80h], xmm2
00007FF6412B3465  movaps       xmm2, xmmword ptr [rsp+970h]
00007FF6412B346D  movaps       xmmword ptr [rsp+A70h], xmm2
00007FF6412B3475  movaps       xmm2, xmmword ptr [rsp+BB0h]
00007FF6412B347D  movaps       xmmword ptr [rsp+A60h], xmm2
00007FF6412B3485  mov          qword ptr [rsp+3D0h], rsi
00007FF6412B348D  movdqu       xmmword ptr [rsp+3D8h], xmm6
00007FF6412B3496  mov          qword ptr [rsp+3E8h], rsi
00007FF6412B349E  movaps       xmm2, xmmword ptr [rsp+BA0h]
00007FF6412B34A6  movups       xmmword ptr [rsp+3F0h], xmm2
00007FF6412B34AE  mov          qword ptr [rsp+400h], rsi
00007FF6412B34B6  movaps       xmm2, xmmword ptr [rsp+B90h]
00007FF6412B34BE  movups       xmmword ptr [rsp+408h], xmm2
00007FF6412B34C6  mov          qword ptr [rsp+418h], rsi
00007FF6412B34CE  movaps       xmm2, xmmword ptr [rsp+B80h]
00007FF6412B34D6  movups       xmmword ptr [rsp+420h], xmm2
00007FF6412B34DE  mov          qword ptr [rsp+430h], rsi
00007FF6412B34E6  movaps       xmm2, xmmword ptr [rsp+B70h]
00007FF6412B34EE  movups       xmmword ptr [rsp+438h], xmm2
00007FF6412B34F6  mov          qword ptr [rsp+448h], rsi
00007FF6412B34FE  movaps       xmm2, xmmword ptr [rsp+B60h]
00007FF6412B3506  movups       xmmword ptr [rsp+450h], xmm2
00007FF6412B350E  mov          qword ptr [rsp+460h], rsi
00007FF6412B3516  movaps       xmm2, xmmword ptr [rsp+B50h]
00007FF6412B351E  movups       xmmword ptr [rsp+468h], xmm2
00007FF6412B3526  mov          qword ptr [rsp+478h], rsi
00007FF6412B352E  movaps       xmm2, xmmword ptr [rsp+B40h]
00007FF6412B3536  movups       xmmword ptr [rsp+480h], xmm2
00007FF6412B353E  mov          qword ptr [rsp+490h], rsi
00007FF6412B3546  movaps       xmm2, xmmword ptr [rsp+B30h]
00007FF6412B354E  movups       xmmword ptr [rsp+498h], xmm2
00007FF6412B3556  mov          qword ptr [rsp+4A8h], rsi
00007FF6412B355E  movaps       xmm2, xmmword ptr [rsp+B20h]
00007FF6412B3566  movups       xmmword ptr [rsp+4B0h], xmm2
00007FF6412B356E  mov          qword ptr [rsp+4C0h], rsi
00007FF6412B3576  movaps       xmm2, xmmword ptr [rsp+B10h]
00007FF6412B357E  movups       xmmword ptr [rsp+4C8h], xmm2
00007FF6412B3586  movups       xmmword ptr [rsp+4D8h], xmm0
00007FF6412B358E  movups       xmmword ptr [rsp+4E8h], xmm1
00007FF6412B3596  lea          rcx, [00007FF6413DDA3Ch]
00007FF6412B359D  mov          qword ptr [rsp+4F8h], rcx
00007FF6412B35A5  mov          qword ptr [rsp+500h], 1h
00007FF6412B35B1  mov          qword ptr [rsp+508h], rax
00007FF6412B35B9  movaps       xmm0, xmmword ptr [rsp+B00h]
00007FF6412B35C1  movups       xmmword ptr [rsp+510h], xmm0
00007FF6412B35C9  mov          qword ptr [rsp+520h], rax
00007FF6412B35D1  movaps       xmm0, xmmword ptr [rsp+AF0h]
00007FF6412B35D9  movups       xmmword ptr [rsp+528h], xmm0
00007FF6412B35E1  mov          qword ptr [rsp+538h], 0h
00007FF6412B35ED  mov          qword ptr [rsp+540h], rbx
00007FF6412B35F5  movaps       xmm0, xmmword ptr [rsp+AE0h]
00007FF6412B35FD  movups       xmmword ptr [rsp+548h], xmm0
00007FF6412B3605  mov          qword ptr [rsp+558h], 0h
00007FF6412B3611  mov          qword ptr [rsp+560h], rbx
00007FF6412B3619  movaps       xmm0, xmmword ptr [rsp+AD0h]
00007FF6412B3621  movups       xmmword ptr [rsp+568h], xmm0
00007FF6412B3629  mov          qword ptr [rsp+578h], 0h
00007FF6412B3635  mov          qword ptr [rsp+580h], rbx
00007FF6412B363D  movaps       xmm0, xmmword ptr [rsp+AC0h]
00007FF6412B3645  movups       xmmword ptr [rsp+588h], xmm0
00007FF6412B364D  mov          qword ptr [rsp+598h], 0h
00007FF6412B3659  mov          qword ptr [rsp+5A0h], rbx
00007FF6412B3661  movaps       xmm0, xmmword ptr [rsp+AB0h]
00007FF6412B3669  movups       xmmword ptr [rsp+5A8h], xmm0
00007FF6412B3671  mov          qword ptr [rsp+5B8h], rsi
00007FF6412B3679  movaps       xmm0, xmmword ptr [rsp+AA0h]
00007FF6412B3681  movups       xmmword ptr [rsp+5C0h], xmm0
00007FF6412B3689  mov          qword ptr [rsp+5D0h], 0h
00007FF6412B3695  mov          qword ptr [rsp+5D8h], rbx
00007FF6412B369D  movaps       xmm0, xmmword ptr [rsp+A90h]
00007FF6412B36A5  movups       xmmword ptr [rsp+5E0h], xmm0
00007FF6412B36AD  mov          qword ptr [rsp+5F0h], rsi
00007FF6412B36B5  movaps       xmm0, xmmword ptr [rsp+A80h]
00007FF6412B36BD  movups       xmmword ptr [rsp+5F8h], xmm0
00007FF6412B36C5  mov          qword ptr [rsp+608h], rsi
00007FF6412B36CD  movaps       xmm0, xmmword ptr [rsp+A70h]
00007FF6412B36D5  movups       xmmword ptr [rsp+610h], xmm0
00007FF6412B36DD  mov          qword ptr [rsp+620h], rsi
00007FF6412B36E5  movaps       xmm0, xmmword ptr [rsp+A60h]
00007FF6412B36ED  movups       xmmword ptr [rsp+628h], xmm0
00007FF6412B36F5  movaps       xmm0, xmmword ptr [__xmm@000000000000001c0001000007230203 (00007FF6413D05A0h)]
00007FF6412B36FC  movups       xmmword ptr [rsp+638h], xmm0
00007FF6412B3704  movaps       xmm0, xmmword ptr [__xmm@00000002000000020000000200000000 (00007FF6413D05B0h)]
00007FF6412B370B  movups       xmmword ptr [rsp+648h], xmm0
00007FF6412B3713  mov          dword ptr [rsp+658h], 1h
00007FF6412B371E  mov          qword ptr [rsp+20h], rdi
00007FF6412B3723  lea          rcx, [rsp+1A0h]
00007FF6412B372B  lea          rdx, [rsp+3D0h]
00007FF6412B3733  lea          r8, [rsp+BC8h]
00007FF6412B373B  lea          r9, [rsp+800h]
00007FF6412B3743  call         static union enum$<core::result::Result<tuple<>, enum$<naga::back::spv::writer::Error>>, 0, 3, Err> naga::back::spv::writer::Writer::write (00007FF641338860h)
00007FF6412B3748  movzx        edi, byte ptr [rsp+1A0h]
00007FF6412B3750  cmp          rdi, 4h
00007FF6412B3754  jne          static void Fabled_Engine::main+3AACh (00007FF6412B674Ch)
00007FF6412B375A  mov          r13, qword ptr [rsp+2D0h]
00007FF6412B3762  mov          r15, qword ptr [rsp+2D8h]
00007FF6412B376A  mov          r14, qword ptr [rsp+2E0h]
00007FF6412B3772  lea          rcx, [rsp+3D0h]
00007FF6412B377A  call         static void core::ptr::drop_in_place<naga::back::spv::writer::Writer> (00007FF6412DDAB0h)
00007FF6412B377F  lea          r12, [r14*4]
00007FF6412B3787  test         r12, r12
00007FF6412B378A  je           static void Fabled_Engine::main+B33h (00007FF6412B37D3h)
00007FF6412B378C  mov          rcx, qword ptr [00007FF6413EF1D8h]
00007FF6412B3793  test         rcx, rcx
00007FF6412B3796  jne          static void Fabled_Engine::main+B10h (00007FF6412B37B0h)
00007FF6412B3798  call         GetProcessHeap (00007FF6413C19C8h)
00007FF6412B379D  test         rax, rax
00007FF6412B37A0  je           static void Fabled_Engine::main+3B9Dh (00007FF6412B683Dh)
00007FF6412B37A6  mov          rcx, rax
00007FF6412B37A9  mov          qword ptr [00007FF6413EF1D8h], rax
00007FF6412B37B0  xor          edx, edx
00007FF6412B37B2  mov          r8, r12
00007FF6412B37B5  call         HeapAlloc (00007FF6413C19CEh)
00007FF6412B37BA  test         rax, rax
00007FF6412B37BD  je           static void Fabled_Engine::main+3B9Dh (00007FF6412B683Dh)
00007FF6412B37C3  mov          rdi, rax
00007FF6412B37C6  mov          rbx, r12
00007FF6412B37C9  test         r14, r14
00007FF6412B37CC  jne          static void Fabled_Engine::main+B43h (00007FF6412B37E3h)
00007FF6412B37CE  jmp          static void Fabled_Engine::main+BCCh (00007FF6412B386Ch)
00007FF6412B37D3  mov          edi, 1h
00007FF6412B37D8  xor          ebx, ebx
00007FF6412B37DA  test         r14, r14
00007FF6412B37DD  je           static void Fabled_Engine::main+BCCh (00007FF6412B386Ch)
00007FF6412B37E3  xor          ebp, ebp
00007FF6412B37E5  lea          r14, [rsp+3D0h]
00007FF6412B37ED  xor          edx, edx
00007FF6412B37EF  jmp          static void Fabled_Engine::main+B78h (00007FF6412B3818h)
00007FF6412B37F1  nop          word ptr cs:[rax+rax*1], ax
00007FF6412B37FB  nop          dword ptr [rax+rax*1], eax
00007FF6412B3800  mov          dword ptr [rdi+rdx*1], esi
00007FF6412B3803  add          rdx, 4h
00007FF6412B3807  mov          rbx, qword ptr [rsp+3D8h]
00007FF6412B380F  add          rbp, 4h
00007FF6412B3813  cmp          r12, rbp
00007FF6412B3816  je           static void Fabled_Engine::main+BCCh (00007FF6412B386Ch)
00007FF6412B3818  mov          rax, r13
00007FF6412B381B  add          rax, rbp
00007FF6412B381E  mov          qword ptr [rsp+3D0h], rdi
00007FF6412B3826  mov          qword ptr [rsp+3D8h], rbx
00007FF6412B382E  mov          qword ptr [rsp+3E0h], rdx
00007FF6412B3836  mov          qword ptr [rsp+3E8h], rax
00007FF6412B383E  mov          esi, dword ptr [r13+rbp*1]
00007FF6412B3843  sub          rbx, rdx
00007FF6412B3846  cmp          rbx, 3h
00007FF6412B384A  ja           static void Fabled_Engine::main+B60h (00007FF6412B3800h)
00007FF6412B384C  mov          r8d, 4h
00007FF6412B3852  mov          rcx, r14
00007FF6412B3855  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<u8,alloc::alloc::Global> (00007FF6413C2C10h)
00007FF6412B385A  mov          rdi, qword ptr [rsp+3D0h]
00007FF6412B3862  mov          rdx, qword ptr [rsp+3E0h]
00007FF6412B386A  jmp          static void Fabled_Engine::main+B60h (00007FF6412B3800h)
00007FF6412B386C  shl          r15, 2h
00007FF6412B3870  test         r15, r15
00007FF6412B3873  je           static void Fabled_Engine::main+BE6h (00007FF6412B3886h)
00007FF6412B3875  mov          rcx, qword ptr [00007FF6413EF1D8h]
00007FF6412B387C  xor          edx, edx
00007FF6412B387E  mov          r8, r13
00007FF6412B3881  call         HeapFree (00007FF6413C19BCh)
00007FF6412B3886  lea          rcx, [rsp+800h]
00007FF6412B388E  call         static void core::ptr::drop_in_place<alloc::vec::Vec<naga::valid::analyzer::FunctionInfo, alloc::alloc::Global>> (00007FF6412B2130h)
00007FF6412B3893  lea          rcx, [rsp+818h]
00007FF6412B389B  call         static void core::ptr::drop_in_place<alloc::vec::Vec<naga::valid::analyzer::FunctionInfo, alloc::alloc::Global>> (00007FF6412B2130h)
00007FF6412B38A0  lea          rcx, [rsp+BC8h]
00007FF6412B38A8  call         static void core::ptr::drop_in_place<naga::Module> (00007FF6412B1160h)
00007FF6412B38AD  lea          rcx, [rsp+3D0h]


//Vert to glsl es
00007FF6FB813424  mov          rcx, rsi
00007FF6FB813427  mov          rdx, r13
00007FF6FB81342A  mov          r8, r15
00007FF6FB81342D  call         static void std::path::Path::components (00007FF6FB9127C0h)
00007FF6FB813432  lea          rcx, [rsp+810h]
00007FF6FB81343A  mov          rdx, rsi
00007FF6FB81343D  call         static void std::path::{{impl}}::next_back (00007FF6FB918AB0h)
00007FF6FB813442  mov          rax, qword ptr [rsp+810h]
00007FF6FB81344A  cmp          rax, 5h
00007FF6FB81344E  je           static void Fabled_Engine::main+5E0Dh (00007FF6FB8190ADh)
00007FF6FB813454  cmp          eax, 4h
00007FF6FB813457  jne          static void Fabled_Engine::main+5E0Dh (00007FF6FB8190ADh)
00007FF6FB81345D  mov          rbx, qword ptr [rsp+818h]
00007FF6FB813465  mov          rdx, qword ptr [rsp+820h]
00007FF6FB81346D  cmp          rdx, 2h
00007FF6FB813471  jne          static void Fabled_Engine::main+1E1h (00007FF6FB813481h)
00007FF6FB813473  movzx        eax, word ptr [rbx]
00007FF6FB813476  cmp          eax, 2E2Eh
00007FF6FB81347B  je           static void Fabled_Engine::main+5E0Dh (00007FF6FB8190ADh)
00007FF6FB813481  lea          rcx, [rbx-1h]
00007FF6FB813485  not          rbx
00007FF6FB813488  xor          eax, eax
00007FF6FB81348A  nop          word ptr [rax+rax*1], ax
00007FF6FB813490  cmp          rdx, rax
00007FF6FB813493  je           static void Fabled_Engine::main+5E0Dh (00007FF6FB8190ADh)
00007FF6FB813499  mov          rsi, rax
00007FF6FB81349C  add          rbx, 1h
00007FF6FB8134A0  cmp          byte ptr [rcx+rdx*1], 2Eh
00007FF6FB8134A4  lea          rcx, [rcx-1h]
00007FF6FB8134A8  lea          rax, [rax+1h]
00007FF6FB8134AC  jne          static void Fabled_Engine::main+1F0h (00007FF6FB813490h)
00007FF6FB8134AE  mov          rcx, rdx
00007FF6FB8134B1  sub          rcx, rax
00007FF6FB8134B4  add          rcx, 1h
00007FF6FB8134B8  cmp          rdx, rcx
00007FF6FB8134BB  jb           static void Fabled_Engine::main+5FFEh (00007FF6FB81929Eh)
00007FF6FB8134C1  mov          rcx, rdx
00007FF6FB8134C4  sub          rcx, rbx
00007FF6FB8134C7  xor          edi, edi
00007FF6FB8134C9  cmp          rdx, rax
00007FF6FB8134CC  cmovne       rdi, rcx
00007FF6FB8134D0  je           static void Fabled_Engine::main+5E0Dh (00007FF6FB8190ADh)
00007FF6FB8134D6  test         rsi, rsi
00007FF6FB8134D9  je           static void Fabled_Engine::main+3BBh (00007FF6FB81365Bh)
00007FF6FB8134DF  lea          rcx, [rdi+rsi*1]
00007FF6FB8134E3  mov          rdx, rdi
00007FF6FB8134E6  jmp          static void Fabled_Engine::main+258h (00007FF6FB8134F8h)
00007FF6FB8134E8  nop          dword ptr [rax+rax*1], eax
00007FF6FB8134F0  mov          rdx, rbx
00007FF6FB8134F3  cmp          rdx, rcx
00007FF6FB8134F6  je           static void Fabled_Engine::main+2BDh (00007FF6FB81355Dh)
00007FF6FB8134F8  lea          rbx, [rdx+1h]
00007FF6FB8134FC  movzx        eax, byte ptr [rdx]
00007FF6FB8134FF  test         al, al
00007FF6FB813501  jns          static void Fabled_Engine::main+250h (00007FF6FB8134F0h)
00007FF6FB813503  add          rdx, 2h
00007FF6FB813507  cmp          al, E0h
00007FF6FB813509  jae          static void Fabled_Engine::main+274h (00007FF6FB813514h)
00007FF6FB81350B  cmp          rbx, rcx
00007FF6FB81350E  cmove        rdx, rbx
00007FF6FB813512  jmp          static void Fabled_Engine::main+253h (00007FF6FB8134F3h)
00007FF6FB813514  cmp          al, EDh
00007FF6FB813516  jne          static void Fabled_Engine::main+297h (00007FF6FB813537h)
00007FF6FB813518  cmp          rbx, rcx
00007FF6FB81351B  cmove        rdx, rbx
00007FF6FB81351F  cmp          rdx, rcx
00007FF6FB813522  je           static void Fabled_Engine::main+2BDh (00007FF6FB81355Dh)
00007FF6FB813524  add          rdx, 1h
00007FF6FB813528  cmp          rbx, rcx
00007FF6FB81352B  je           static void Fabled_Engine::main+253h (00007FF6FB8134F3h)
00007FF6FB81352D  cmp          byte ptr [rbx], A0h
00007FF6FB813530  jb           static void Fabled_Engine::main+253h (00007FF6FB8134F3h)
00007FF6FB813532  jmp          static void Fabled_Engine::main+5FE4h (00007FF6FB819284h)
00007FF6FB813537  cmp          rbx, rcx
00007FF6FB81353A  mov          rbp, rdx
00007FF6FB81353D  cmove        rbp, rbx
00007FF6FB813541  lea          rdx, [rbp+1h]
00007FF6FB813545  cmp          rbp, rcx
00007FF6FB813548  cmove        rdx, rbp
00007FF6FB81354C  cmp          al, F0h
00007FF6FB81354E  jb           static void Fabled_Engine::main+253h (00007FF6FB8134F3h)
00007FF6FB813550  lea          rax, [rdx+1h]
00007FF6FB813554  cmp          rdx, rcx
00007FF6FB813557  cmovne       rdx, rax
00007FF6FB81355B  jmp          static void Fabled_Engine::main+253h (00007FF6FB8134F3h)
00007FF6FB81355D  cmp          rsi, 3h
00007FF6FB813561  je           static void Fabled_Engine::main+3A3h (00007FF6FB813643h)
00007FF6FB813567  cmp          rsi, 4h
00007FF6FB81356B  jne          static void Fabled_Engine::main+3BBh (00007FF6FB81365Bh)
00007FF6FB813571  cmp          dword ptr [rdi], 6C736777h
00007FF6FB813577  je           static void Fabled_Engine::main+2F2Ch (00007FF6FB8161CCh)
00007FF6FB81357D  cmp          dword ptr [rdi], 74726576h
00007FF6FB813583  je           static void Fabled_Engine::main+2F9h (00007FF6FB813599h)
00007FF6FB813585  cmp          dword ptr [rdi], 67617266h
00007FF6FB81358B  je           static void Fabled_Engine::main+2F9h (00007FF6FB813599h)
00007FF6FB81358D  cmp          dword ptr [rdi], 706D6F63h
00007FF6FB813593  jne          static void Fabled_Engine::main+3BBh (00007FF6FB81365Bh)
00007FF6FB813599  mov          qword ptr [rsp+30h], r12
00007FF6FB81359E  lea          rcx, [rsp+2C0h]
00007FF6FB8135A6  mov          rdx, r13
00007FF6FB8135A9  mov          r8, r15
00007FF6FB8135AC  call         static void std::fs::read_to_string::inner (00007FF6FB9180B0h)
00007FF6FB8135B1  cmp          dword ptr [rsp+2C0h], 1h
00007FF6FB8135B9  mov          rbx, qword ptr [rsp+2D0h]
00007FF6FB8135C1  mov          rsi, qword ptr [rsp+2C8h]
00007FF6FB8135C9  mov          qword ptr [rsp+B8h], r13
00007FF6FB8135D1  je           static void Fabled_Engine::main+61D9h (00007FF6FB819479h)
00007FF6FB8135D7  mov          r12, qword ptr [rsp+2D8h]
00007FF6FB8135DF  pxor         xmm0, xmm0
00007FF6FB8135E3  movdqu       xmmword ptr [rsp+260h], xmm0
00007FF6FB8135EC  mov          qword ptr [rsp+250h], 0h
00007FF6FB8135F8  lea          rax, [00007FF6FB934260h]
00007FF6FB8135FF  mov          qword ptr [rsp+258h], rax
00007FF6FB813607  cmp          dword ptr [rdi], 74726576h
00007FF6FB81360D  je           static void Fabled_Engine::main+31F9h (00007FF6FB816499h)
00007FF6FB813613  cmp          dword ptr [rdi], 67617266h
00007FF6FB813619  je           static void Fabled_Engine::main+320Ah (00007FF6FB8164AAh)
00007FF6FB81361F  mov          r14b, 2h
00007FF6FB813622  cmp          dword ptr [rdi], 706D6F63h
00007FF6FB813628  jne          static void Fabled_Engine::main+6465h (00007FF6FB819705h)
00007FF6FB81362E  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB813635  test         rcx, rcx
00007FF6FB813638  jne          static void Fabled_Engine::main+3231h (00007FF6FB8164D1h)
00007FF6FB81363E  jmp          static void Fabled_Engine::main+3219h (00007FF6FB8164B9h)
00007FF6FB813643  movzx        eax, word ptr [rdi]
00007FF6FB813646  xor          eax, 7073h
00007FF6FB81364B  movzx        ecx, byte ptr [rdi+2h]
00007FF6FB81364F  xor          ecx, 76h
00007FF6FB813652  or           cx, ax
00007FF6FB813655  je           static void Fabled_Engine::main+50E2h (00007FF6FB818382h)
00007FF6FB81365B  mov          rax, qword ptr [00007FF6FB945DF8h]
00007FF6FB813662  pxor         xmm0, xmm0
00007FF6FB813666  movdqu       xmmword ptr [rsp+978h], xmm0
00007FF6FB81366F  movdqa       xmmword ptr [rsp+990h], xmm0
00007FF6FB813678  movdqu       xmmword ptr [rsp+9A8h], xmm0
00007FF6FB813681  movdqa       xmmword ptr [rsp+9C0h], xmm0
00007FF6FB81368A  movdqu       xmmword ptr [rsp+9D8h], xmm0
00007FF6FB813693  mov          qword ptr [rsp+970h], rax
00007FF6FB81369B  mov          qword ptr [rsp+988h], rax
00007FF6FB8136A3  mov          qword ptr [rsp+9A0h], rax
00007FF6FB8136AB  mov          qword ptr [rsp+9B8h], rax
00007FF6FB8136B3  mov          qword ptr [rsp+9D0h], rax
00007FF6FB8136BB  lea          rcx, [rsp+2C0h]
00007FF6FB8136C3  lea          rdx, [rsp+970h]
00007FF6FB8136CB  call         static union enum$<core::result::Result<naga::valid::ModuleInfo, anyhow::Error>> fabled_render::shader::shader_validator::{{impl}}::validate (00007FF6FB835AE0h)
00007FF6FB8136D0  cmp          dword ptr [rsp+2C0h], 1h
00007FF6FB8136D8  mov          rdi, qword ptr [rsp+2C8h]
00007FF6FB8136E0  je           static void Fabled_Engine::main+600Ch (00007FF6FB8192ACh)
00007FF6FB8136E6  mov          rax, qword ptr [rsp+2F0h]
00007FF6FB8136EE  mov          qword ptr [rsp+838h], rax
00007FF6FB8136F6  movups       xmm0, xmmword ptr [rsp+2D0h]
00007FF6FB8136FE  movups       xmm1, xmmword ptr [rsp+2E0h]
00007FF6FB813706  movups       xmmword ptr [rsp+828h], xmm1
00007FF6FB81370E  movups       xmmword ptr [rsp+818h], xmm0
00007FF6FB813716  mov          qword ptr [rsp+810h], rdi
00007FF6FB81371E  lea          rcx, [rsp+810h]
00007FF6FB813726  call         static void core::ptr::drop_in_place<alloc::vec::Vec<naga::valid::analyzer::FunctionInfo, alloc::alloc::Global>> (00007FF6FB812060h)
00007FF6FB81372B  lea          rcx, [rsp+828h]
00007FF6FB813733  call         static void core::ptr::drop_in_place<alloc::vec::Vec<naga::valid::analyzer::FunctionInfo, alloc::alloc::Global>> (00007FF6FB812060h)
00007FF6FB813738  mov          rsi, qword ptr [rsp+970h]
00007FF6FB813740  movups       xmm0, xmmword ptr [rsp+978h]
00007FF6FB813748  movaps       xmmword ptr [rsp+1C0h], xmm0
00007FF6FB813750  movups       xmm0, xmmword ptr [rsp+988h]
00007FF6FB813758  movaps       xmmword ptr [rsp+1D0h], xmm0
00007FF6FB813760  movups       xmm0, xmmword ptr [rsp+998h]
00007FF6FB813768  movaps       xmmword ptr [rsp+1E0h], xmm0
00007FF6FB813770  movups       xmm0, xmmword ptr [rsp+9A8h]
00007FF6FB813778  movaps       xmmword ptr [rsp+1F0h], xmm0
00007FF6FB813780  movups       xmm0, xmmword ptr [rsp+9B8h]
00007FF6FB813788  movaps       xmmword ptr [rsp+200h], xmm0
00007FF6FB813790  movups       xmm0, xmmword ptr [rsp+9C8h]
00007FF6FB813798  movaps       xmmword ptr [rsp+210h], xmm0
00007FF6FB8137A0  movups       xmm0, xmmword ptr [rsp+9D8h]
00007FF6FB8137A8  movaps       xmmword ptr [rsp+220h], xmm0
00007FF6FB8137B0  test         r12, r12
00007FF6FB8137B3  je           static void Fabled_Engine::main+526h (00007FF6FB8137C6h)
00007FF6FB8137B5  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB8137BC  xor          edx, edx
00007FF6FB8137BE  mov          r8, r13
00007FF6FB8137C1  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB8137C6  movaps       xmm0, xmmword ptr [rsp+220h]
00007FF6FB8137CE  movups       xmmword ptr [rsp+AD0h], xmm0
00007FF6FB8137D6  movaps       xmm0, xmmword ptr [rsp+210h]
00007FF6FB8137DE  movups       xmmword ptr [rsp+AC0h], xmm0
00007FF6FB8137E6  movaps       xmm0, xmmword ptr [rsp+200h]
00007FF6FB8137EE  movups       xmmword ptr [rsp+AB0h], xmm0
00007FF6FB8137F6  movdqa       xmm0, xmmword ptr [rsp+1C0h]
00007FF6FB8137FF  movapd       xmm1, xmmword ptr [rsp+1D0h]
00007FF6FB813808  movdqa       xmm2, xmmword ptr [rsp+1E0h]
00007FF6FB813811  movdqa       xmm3, xmmword ptr [rsp+1F0h]
00007FF6FB81381A  movdqu       xmmword ptr [rsp+AA0h], xmm3
00007FF6FB813823  movdqu       xmmword ptr [rsp+A90h], xmm2
00007FF6FB81382C  movupd       xmmword ptr [rsp+A80h], xmm1
00007FF6FB813835  movdqu       xmmword ptr [rsp+A70h], xmm0
00007FF6FB81383E  mov          qword ptr [rsp+A68h], rsi
00007FF6FB813846  lea          rcx, [rsp+2C0h]
00007FF6FB81384E  lea          rdx, [rsp+A68h]
00007FF6FB813856  call         static union enum$<core::result::Result<naga::valid::ModuleInfo, anyhow::Error>> fabled_render::shader::shader_validator::{{impl}}::validate (00007FF6FB835AE0h)
00007FF6FB81385B  cmp          dword ptr [rsp+2C0h], 1h
00007FF6FB813863  mov          rdi, qword ptr [rsp+2C8h]
00007FF6FB81386B  je           static void Fabled_Engine::main+5D7Dh (00007FF6FB81901Dh)
00007FF6FB813871  mov          rax, qword ptr [rsp+2F0h]
00007FF6FB813879  mov          qword ptr [rsp+178h], rax
00007FF6FB813881  movdqu       xmm0, xmmword ptr [rsp+2D0h]
00007FF6FB81388A  movupd       xmm1, xmmword ptr [rsp+2E0h]
00007FF6FB813893  movupd       xmmword ptr [rsp+168h], xmm1
00007FF6FB81389C  movdqu       xmmword ptr [rsp+158h], xmm0
00007FF6FB8138A5  mov          qword ptr [rsp+150h], rdi
00007FF6FB8138AD  mov          rsi, qword ptr [00007FF6FB945F50h]
00007FF6FB8138B4  xorps        xmm8, xmm8
00007FF6FB8138B8  movups       xmmword ptr [rsp+5F8h], xmm8
00007FF6FB8138C1  mov          qword ptr [rsp+5F0h], rsi
00007FF6FB8138C9  mov          rax, qword ptr [rsp+AD8h]
00007FF6FB8138D1  test         rax, rax
00007FF6FB8138D4  je           static void Fabled_Engine::main+2E5Fh (00007FF6FB8160FFh)
00007FF6FB8138DA  mov          r12, qword ptr [rsp+AC8h]
00007FF6FB8138E2  imul         rax, rax, D0h
00007FF6FB8138E9  add          rax, r12
00007FF6FB8138EC  mov          qword ptr [rsp+C0h], rax
00007FF6FB8138F4  pcmpeqd      xmm9, xmm9
00007FF6FB8138F9  movapd       xmm10, xmmword ptr [__xmm@7fffffffffffffff7fffffffffffffff (00007FF6FB92C5A0h)]
00007FF6FB813902  xorpd        xmm11, xmm11
00007FF6FB813907  movsd        xmm12, qword ptr [__real@4059000000000000 (00007FF6FB92C5B0h)]
00007FF6FB813910  movsd        xmm13, qword ptr [__real@c060000000000000 (00007FF6FB92C5B8h)]
00007FF6FB813919  movsd        xmm14, qword ptr [__real@405fc00000000000 (00007FF6FB92C5C0h)]
00007FF6FB813922  mov          r14b, byte ptr [r12+CCh]
00007FF6FB81392A  mov          rbx, qword ptr [r12]
00007FF6FB81392E  mov          rsi, qword ptr [r12+10h]
00007FF6FB813933  test         rsi, rsi
00007FF6FB813936  mov          qword ptr [rsp+30h], r12
00007FF6FB81393B  jne          static void Fabled_Engine::main+6B0h (00007FF6FB813950h)
00007FF6FB81393D  mov          edi, 1h
00007FF6FB813942  xor          ebp, ebp
00007FF6FB813944  jmp          static void Fabled_Engine::main+6EDh (00007FF6FB81398Dh)
00007FF6FB813946  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB813950  mov          rax, qword ptr [00007FF6FB94E1D8h]
00007FF6FB813957  test         rax, rax
00007FF6FB81395A  jne          static void Fabled_Engine::main+6D1h (00007FF6FB813971h)
00007FF6FB81395C  call         GetProcessHeap (00007FF6FB91D5C8h)
00007FF6FB813961  test         rax, rax
00007FF6FB813964  je           static void Fabled_Engine::main+5EA6h (00007FF6FB819146h)
00007FF6FB81396A  mov          qword ptr [00007FF6FB94E1D8h], rax
00007FF6FB813971  mov          rcx, rax
00007FF6FB813974  xor          edx, edx
00007FF6FB813976  mov          r8, rsi
00007FF6FB813979  call         HeapAlloc (00007FF6FB91D5CEh)
00007FF6FB81397E  test         rax, rax
00007FF6FB813981  je           static void Fabled_Engine::main+5EA6h (00007FF6FB819146h)
00007FF6FB813987  mov          rdi, rax
00007FF6FB81398A  mov          rbp, rsi
00007FF6FB81398D  mov          rcx, rdi
00007FF6FB813990  mov          rdx, rbx
00007FF6FB813993  mov          r8, rsi
00007FF6FB813996  call         memcpy (00007FF6FB91E6A7h)
00007FF6FB81399B  mov          dword ptr [rsp+288h], 1360001h
00007FF6FB8139A6  mov          byte ptr [rsp+28Ch], r14b
00007FF6FB8139AE  mov          qword ptr [rsp+270h], rdi
00007FF6FB8139B6  mov          qword ptr [rsp+278h], rbp
00007FF6FB8139BE  mov          qword ptr [rsp+280h], rsi
00007FF6FB8139C6  lea          rcx, [rsp+AC8h]
00007FF6FB8139CE  call         static struct slice<u8> alloc::vec::{{impl}}::deref<u8,alloc::alloc::Global> (00007FF6FB812DA0h)
00007FF6FB8139D3  test         rdx, rdx
00007FF6FB8139D6  je           static void Fabled_Engine::main+5BDEh (00007FF6FB818E7Eh)
00007FF6FB8139DC  mov          rsi, rax
00007FF6FB8139DF  mov          rbp, rdx
00007FF6FB8139E2  add          qword ptr [rsp+30h], D0h
00007FF6FB8139EB  shl          rbp, 4h
00007FF6FB8139EF  add          rbp, FFFFFFFFFFFFFFF0h
00007FF6FB8139F3  mov          bl, byte ptr [rsp+28Ch]
00007FF6FB8139FA  mov          r14, qword ptr [rsp+270h]
00007FF6FB813A02  mov          rdi, qword ptr [rsp+280h]
00007FF6FB813A0A  shr          rbp, 4h
00007FF6FB813A0E  add          rbp, 1h
00007FF6FB813A12  xor          r13d, r13d
00007FF6FB813A15  xor          r12d, r12d
00007FF6FB813A18  jmp          static void Fabled_Engine::main+794h (00007FF6FB813A34h)
00007FF6FB813A1A  nop          word ptr [rax+rax*1], ax
00007FF6FB813A20  add          r12, 1h
00007FF6FB813A24  add          r13, D0h
00007FF6FB813A2B  cmp          rbp, r12
00007FF6FB813A2E  je           static void Fabled_Engine::main+5BDEh (00007FF6FB818E7Eh)
00007FF6FB813A34  cmp          bl, byte ptr [rsi+r13*1+CCh]
00007FF6FB813A3C  jne          static void Fabled_Engine::main+780h (00007FF6FB813A20h)
00007FF6FB813A3E  cmp          rdi, qword ptr [rsi+r13*1+10h]
00007FF6FB813A43  jne          static void Fabled_Engine::main+780h (00007FF6FB813A20h)
00007FF6FB813A45  mov          rdx, qword ptr [rsi+r13*1]
00007FF6FB813A49  mov          rcx, r14
00007FF6FB813A4C  mov          r8, rdi
00007FF6FB813A4F  call         memcmp (00007FF6FB91E6ADh)
00007FF6FB813A54  test         eax, eax
00007FF6FB813A56  jne          static void Fabled_Engine::main+780h (00007FF6FB813A20h)
00007FF6FB813A58  lea          rax, [rsp+630h]
00007FF6FB813A60  movups       xmmword ptr [rax], xmm8
00007FF6FB813A64  mov          qword ptr [rsp+620h], 0h
00007FF6FB813A70  lea          rcx, [00007FF6FB934260h]
00007FF6FB813A77  mov          qword ptr [rsp+628h], rcx
00007FF6FB813A7F  lea          rax, [rsp+1D0h]
00007FF6FB813A87  movups       xmmword ptr [rax], xmm8
00007FF6FB813A8B  mov          qword ptr [rsp+1C0h], 0h
00007FF6FB813A97  mov          qword ptr [rsp+1C8h], rcx
00007FF6FB813A9F  movups       xmmword ptr [rax+20h], xmm8
00007FF6FB813AA4  mov          qword ptr [rsp+1E0h], 0h
00007FF6FB813AB0  mov          qword ptr [rsp+1E8h], rcx
00007FF6FB813AB8  mov          rcx, qword ptr [00007FF6FB945DF8h]
00007FF6FB813ABF  mov          qword ptr [rsp+200h], rcx
00007FF6FB813AC7  movups       xmmword ptr [rax+38h], xmm8
00007FF6FB813ACC  mov          dword ptr [rax+48h], 0h
00007FF6FB813AD3  mov          qword ptr [rsp+210h], 0h
00007FF6FB813ADF  mov          r8d, 1h
00007FF6FB813AE5  lea          rcx, [rsp+200h]
00007FF6FB813AED  xor          edx, edx
00007FF6FB813AEF  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<alloc::string::String,alloc::alloc::Global> (00007FF6FB921590h)
00007FF6FB813AF4  mov          rsi, qword ptr [rsp+200h]
00007FF6FB813AFC  mov          rdi, qword ptr [rsp+210h]
00007FF6FB813B04  lea          rax, [rsp+48h]
00007FF6FB813B09  movups       xmmword ptr [rax], xmm8
00007FF6FB813B0D  mov          rax, qword ptr [00007FF6FB945F50h]
00007FF6FB813B14  mov          qword ptr [rsp+40h], rax
00007FF6FB813B19  mov          rax, 2000000000h
00007FF6FB813B23  mov          qword ptr [rsp+2F0h], rax
00007FF6FB813B2B  mov          byte ptr [rsp+2F8h], 3h
00007FF6FB813B33  mov          qword ptr [rsp+2C0h], 0h
00007FF6FB813B3F  mov          qword ptr [rsp+2D0h], 0h
00007FF6FB813B4B  lea          rax, [rsp+40h]
00007FF6FB813B50  mov          qword ptr [rsp+2E0h], rax
00007FF6FB813B58  lea          rax, [00007FF6FB934270h]
00007FF6FB813B5F  mov          qword ptr [rsp+2E8h], rax
00007FF6FB813B67  mov          r8d, 3h
00007FF6FB813B6D  lea          rbx, [rsp+2C0h]
00007FF6FB813B75  mov          rcx, rbx
00007FF6FB813B78  lea          rdx, [00007FF6FB92E460h]
00007FF6FB813B7F  call         static void core::fmt::Formatter::pad (00007FF6FB82BB90h)
00007FF6FB813B84  test         al, al
00007FF6FB813B86  jne          static void Fabled_Engine::main+5DDFh (00007FF6FB81907Fh)
00007FF6FB813B8C  lea          rax, [rdi+1h]
00007FF6FB813B90  lea          rcx, [rdi+rdi*2]
00007FF6FB813B94  movdqu       xmm0, xmmword ptr [rsp+40h]
00007FF6FB813B9A  mov          rdx, qword ptr [rsp+50h]
00007FF6FB813B9F  mov          qword ptr [rsi+rcx*8+10h], rdx
00007FF6FB813BA4  movdqu       xmmword ptr [rsi+rcx*8], xmm0
00007FF6FB813BA9  mov          qword ptr [rsp+210h], rax
00007FF6FB813BB1  cmp          qword ptr [rsp+1D8h], 0h
00007FF6FB813BBA  je           static void Fabled_Engine::main+B90h (00007FF6FB813E30h)
00007FF6FB813BC0  mov          r14, qword ptr [rsp+1C0h]
00007FF6FB813BC8  mov          rdi, qword ptr [rsp+1C8h]
00007FF6FB813BD0  lea          rbx, [r14+rdi*1]
00007FF6FB813BD4  add          rbx, 1h
00007FF6FB813BD8  movdqa       xmm0, xmmword ptr [rdi]
00007FF6FB813BDC  pmovmskb     eax, xmm0
00007FF6FB813BE0  not          eax
00007FF6FB813BE2  lea          rbp, [rdi+10h]
00007FF6FB813BE6  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB813BF0  test         ax, ax
00007FF6FB813BF3  je           static void Fabled_Engine::main+960h (00007FF6FB813C00h)
00007FF6FB813BF5  lea          esi, [rax-1h]
00007FF6FB813BF8  and          esi, eax
00007FF6FB813BFA  jmp          static void Fabled_Engine::main+98Ch (00007FF6FB813C2Ch)
00007FF6FB813BFC  nop          dword ptr [rax], eax
00007FF6FB813C00  cmp          rbp, rbx
00007FF6FB813C03  jae          static void Fabled_Engine::main+9C0h (00007FF6FB813C60h)
00007FF6FB813C05  movdqa       xmm0, xmmword ptr [rbp]
00007FF6FB813C0A  pmovmskb     eax, xmm0
00007FF6FB813C0E  add          rdi, FFFFFFFFFFFFFD80h
00007FF6FB813C15  add          rbp, 10h
00007FF6FB813C19  cmp          ax, FFFFh
00007FF6FB813C1D  je           static void Fabled_Engine::main+960h (00007FF6FB813C00h)
00007FF6FB813C1F  mov          ecx, FFFFFFFEh
00007FF6FB813C24  sub          ecx, eax
00007FF6FB813C26  not          eax
00007FF6FB813C28  mov          esi, eax
00007FF6FB813C2A  and          esi, ecx
00007FF6FB813C2C  bsf          ax, ax
00007FF6FB813C30  movzx        eax, ax
00007FF6FB813C33  neg          rax
00007FF6FB813C36  lea          rcx, [rax+rax*4]
00007FF6FB813C3A  cmp          qword ptr [rdi+rcx*8-20h], 0h
00007FF6FB813C40  mov          eax, esi
00007FF6FB813C42  je           static void Fabled_Engine::main+950h (00007FF6FB813BF0h)
00007FF6FB813C44  mov          r8, qword ptr [rdi+rcx*8-28h]
00007FF6FB813C49  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB813C50  xor          edx, edx
00007FF6FB813C52  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB813C57  mov          eax, esi
00007FF6FB813C59  jmp          static void Fabled_Engine::main+950h (00007FF6FB813BF0h)
00007FF6FB813C5B  nop          dword ptr [rax+rax*1], eax
00007FF6FB813C60  lea          rbx, [rsp+2C0h]
00007FF6FB813C68  test         r14, r14
00007FF6FB813C6B  je           static void Fabled_Engine::main+9E0h (00007FF6FB813C80h)
00007FF6FB813C6D  mov          rcx, qword ptr [rsp+1C8h]
00007FF6FB813C75  lea          r8, [r14+11h]
00007FF6FB813C79  mov          dl, FFh
00007FF6FB813C7B  call         memset (00007FF6FB91E6B3h)
00007FF6FB813C80  lea          rax, [r14+1h]
00007FF6FB813C84  shr          rax, 3h
00007FF6FB813C88  lea          rcx, [rax*8]
00007FF6FB813C90  sub          rcx, rax
00007FF6FB813C93  cmp          r14, 8h
00007FF6FB813C97  cmovb        rcx, r14
00007FF6FB813C9B  mov          qword ptr [rsp+1D8h], 0h
00007FF6FB813CA7  mov          qword ptr [rsp+1D0h], rcx
00007FF6FB813CAF  cmp          qword ptr [rsp+1F8h], 0h
00007FF6FB813CB8  je           static void Fabled_Engine::main+BB0h (00007FF6FB813E50h)
00007FF6FB813CBE  mov          r15, qword ptr [rsp+1E0h]
00007FF6FB813CC6  mov          rdi, qword ptr [rsp+1E8h]
00007FF6FB813CCE  lea          rbx, [r15+rdi*1]
00007FF6FB813CD2  add          rbx, 1h
00007FF6FB813CD6  movdqa       xmm0, xmmword ptr [rdi]
00007FF6FB813CDA  pmovmskb     ecx, xmm0
00007FF6FB813CDE  lea          rbp, [rdi+10h]
00007FF6FB813CE2  cmp          cx, FFFFh
00007FF6FB813CE6  je           static void Fabled_Engine::main+A60h (00007FF6FB813D00h)
00007FF6FB813CE8  mov          eax, ecx
00007FF6FB813CEA  not          eax
00007FF6FB813CEC  mov          edx, FFFFFFFEh
00007FF6FB813CF1  sub          edx, ecx
00007FF6FB813CF3  mov          esi, eax
00007FF6FB813CF5  and          esi, edx
00007FF6FB813CF7  jmp          static void Fabled_Engine::main+A90h (00007FF6FB813D30h)
00007FF6FB813CF9  nop          dword ptr [rax], eax
00007FF6FB813D00  cmp          rbp, rbx
00007FF6FB813D03  jae          static void Fabled_Engine::main+B20h (00007FF6FB813DC0h)
00007FF6FB813D09  movdqa       xmm0, xmmword ptr [rbp]
00007FF6FB813D0E  pmovmskb     eax, xmm0
00007FF6FB813D12  add          rdi, FFFFFFFFFFFFFE80h
00007FF6FB813D19  add          rbp, 10h
00007FF6FB813D1D  cmp          ax, FFFFh
00007FF6FB813D21  je           static void Fabled_Engine::main+A60h (00007FF6FB813D00h)
00007FF6FB813D23  mov          ecx, FFFFFFFEh
00007FF6FB813D28  sub          ecx, eax
00007FF6FB813D2A  not          eax
00007FF6FB813D2C  mov          esi, eax
00007FF6FB813D2E  and          esi, ecx
00007FF6FB813D30  bsf          ax, ax
00007FF6FB813D34  movzx        eax, ax
00007FF6FB813D37  neg          rax
00007FF6FB813D3A  lea          rax, [rax+rax*2]
00007FF6FB813D3E  lea          rcx, [rdi+rax*8]
00007FF6FB813D42  cmp          qword ptr [rcx-10h], 0h
00007FF6FB813D47  je           static void Fabled_Engine::main+AC2h (00007FF6FB813D62h)
00007FF6FB813D49  nop          dword ptr [rax], eax
00007FF6FB813D50  mov          r8, qword ptr [rcx-18h]
00007FF6FB813D54  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB813D5B  xor          edx, edx
00007FF6FB813D5D  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB813D62  test         si, si
00007FF6FB813D65  je           static void Fabled_Engine::main+AD0h (00007FF6FB813D70h)
00007FF6FB813D67  lea          eax, [rsi-1h]
00007FF6FB813D6A  and          eax, esi
00007FF6FB813D6C  jmp          static void Fabled_Engine::main+AFCh (00007FF6FB813D9Ch)
00007FF6FB813D6E  nop
00007FF6FB813D70  cmp          rbp, rbx
00007FF6FB813D73  jae          static void Fabled_Engine::main+B20h (00007FF6FB813DC0h)
00007FF6FB813D75  movdqa       xmm0, xmmword ptr [rbp]
00007FF6FB813D7A  pmovmskb     esi, xmm0
00007FF6FB813D7E  add          rdi, FFFFFFFFFFFFFE80h
00007FF6FB813D85  add          rbp, 10h
00007FF6FB813D89  cmp          si, FFFFh
00007FF6FB813D8D  je           static void Fabled_Engine::main+AD0h (00007FF6FB813D70h)
00007FF6FB813D8F  mov          ecx, FFFFFFFEh
00007FF6FB813D94  sub          ecx, esi
00007FF6FB813D96  not          esi
00007FF6FB813D98  mov          eax, esi
00007FF6FB813D9A  and          eax, ecx
00007FF6FB813D9C  bsf          cx, si
00007FF6FB813DA0  movzx        ecx, cx
00007FF6FB813DA3  neg          rcx
00007FF6FB813DA6  lea          rcx, [rcx+rcx*2]
00007FF6FB813DAA  lea          rcx, [rdi+rcx*8]
00007FF6FB813DAE  mov          esi, eax
00007FF6FB813DB0  cmp          qword ptr [rcx-10h], 0h
00007FF6FB813DB5  jne          static void Fabled_Engine::main+AB0h (00007FF6FB813D50h)
00007FF6FB813DB7  jmp          static void Fabled_Engine::main+AC2h (00007FF6FB813D62h)
00007FF6FB813DB9  nop          dword ptr [rax], eax
00007FF6FB813DC0  lea          rbx, [rsp+2C0h]
00007FF6FB813DC8  test         r15, r15
00007FF6FB813DCB  je           static void Fabled_Engine::main+B40h (00007FF6FB813DE0h)
00007FF6FB813DCD  mov          rcx, qword ptr [rsp+1E8h]
00007FF6FB813DD5  lea          r8, [r15+11h]
00007FF6FB813DD9  mov          dl, FFh
00007FF6FB813DDB  call         memset (00007FF6FB91E6B3h)
00007FF6FB813DE0  mov          qword ptr [rsp+1F8h], 0h
00007FF6FB813DEC  lea          rax, [r15+1h]
00007FF6FB813DF0  shr          rax, 3h
00007FF6FB813DF4  lea          rcx, [rax*8]
00007FF6FB813DFC  sub          rcx, rax
00007FF6FB813DFF  cmp          r15, 8h
00007FF6FB813E03  cmovb        rcx, r15
00007FF6FB813E07  mov          qword ptr [rsp+1F0h], rcx
00007FF6FB813E0F  cmp          rcx, C9h
00007FF6FB813E16  jbe          static void Fabled_Engine::main+BC6h (00007FF6FB813E66h)
00007FF6FB813E18  lea          rdi, [00007FF6FB92D710h]
00007FF6FB813E1F  mov          rsi, rdi
00007FF6FB813E22  jmp          static void Fabled_Engine::main+C65h (00007FF6FB813F05h)
00007FF6FB813E27  nop          word ptr [rax+rax*1], ax
00007FF6FB813E30  mov          r14, qword ptr [rsp+1C0h]
00007FF6FB813E38  test         r14, r14
00007FF6FB813E3B  jne          static void Fabled_Engine::main+9CDh (00007FF6FB813C6Dh)
00007FF6FB813E41  jmp          static void Fabled_Engine::main+9E0h (00007FF6FB813C80h)
00007FF6FB813E46  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB813E50  mov          r15, qword ptr [rsp+1E0h]
00007FF6FB813E58  test         r15, r15
00007FF6FB813E5B  jne          static void Fabled_Engine::main+B2Dh (00007FF6FB813DCDh)
00007FF6FB813E61  jmp          static void Fabled_Engine::main+B40h (00007FF6FB813DE0h)
00007FF6FB813E66  mov          r8d, CAh
00007FF6FB813E6C  mov          rcx, rbx
00007FF6FB813E6F  lea          rdx, [rsp+1E0h]
00007FF6FB813E77  call         static union enum$<core::result::Result<tuple<>, enum$<hashbrown::TryReserveError, 1, 18446744073709551615, AllocError>>> hashbrown::raw::RawTable<tuple<alloc::string::String, tuple<>>, alloc::alloc::Global>::reserve_rehash<tuple<alloc::string::String, tuple<>>,alloc::alloc::Global,closure-0> (00007FF6FB927F70h)
00007FF6FB813E7C  jmp          static void Fabled_Engine::main+B78h (00007FF6FB813E18h)
00007FF6FB813E7E  bsf          dx, si
00007FF6FB813E82  movzx        edx, dx
00007FF6FB813E85  add          rdx, rcx
00007FF6FB813E88  and          rdx, rbp
00007FF6FB813E8B  cmp          byte ptr [rax+rdx*1], 0h
00007FF6FB813E8F  jns          static void Fabled_Engine::main+FE4h (00007FF6FB814284h)
00007FF6FB813E95  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB813E9F  nop
00007FF6FB813EA0  neg          bl
00007FF6FB813EA2  movsx        rcx, bl
00007FF6FB813EA6  add          qword ptr [rsp+1F0h], rcx
00007FF6FB813EAE  shr          r15, 39h
00007FF6FB813EB2  lea          rcx, [rdx-10h]
00007FF6FB813EB6  and          rcx, rbp
00007FF6FB813EB9  mov          byte ptr [rax+rdx*1], r15b
00007FF6FB813EBD  mov          byte ptr [rcx+rax*1+10h], r15b
00007FF6FB813EC2  add          qword ptr [rsp+1F8h], 1h
00007FF6FB813ECB  neg          rdx
00007FF6FB813ECE  lea          rcx, [rdx+rdx*2]
00007FF6FB813ED2  mov          rdx, qword ptr [rsp+50h]
00007FF6FB813ED7  mov          qword ptr [rax+rcx*8-8h], rdx
00007FF6FB813EDC  movdqa       xmm0, xmmword ptr [rsp+40h]
00007FF6FB813EE2  movdqu       xmmword ptr [rax+rcx*8-18h], xmm0
00007FF6FB813EE8  lea          rax, [00007FF6FB92E3B0h]
00007FF6FB813EEF  mov          rsi, qword ptr [rsp+28h]
00007FF6FB813EF4  cmp          rsi, rax
00007FF6FB813EF7  lea          rbx, [rsp+2C0h]
00007FF6FB813EFF  je           static void Fabled_Engine::main+1000h (00007FF6FB8142A0h)
00007FF6FB813F05  mov          rdx, qword ptr [rsi]
00007FF6FB813F08  mov          r8, qword ptr [rsi+8h]
00007FF6FB813F0C  lea          rax, [rsp+108h]
00007FF6FB813F14  movups       xmmword ptr [rax], xmm8
00007FF6FB813F18  mov          rax, qword ptr [00007FF6FB945F50h]
00007FF6FB813F1F  mov          qword ptr [rsp+100h], rax
00007FF6FB813F27  mov          rax, 2000000000h
00007FF6FB813F31  mov          qword ptr [rsp+2F0h], rax
00007FF6FB813F39  mov          byte ptr [rsp+2F8h], 3h
00007FF6FB813F41  mov          qword ptr [rsp+2C0h], 0h
00007FF6FB813F4D  mov          qword ptr [rsp+2D0h], 0h
00007FF6FB813F59  lea          rax, [rsp+100h]
00007FF6FB813F61  mov          qword ptr [rsp+2E0h], rax
00007FF6FB813F69  lea          rax, [00007FF6FB934270h]
00007FF6FB813F70  mov          qword ptr [rsp+2E8h], rax
00007FF6FB813F78  mov          rcx, rbx
00007FF6FB813F7B  call         static void core::fmt::Formatter::pad (00007FF6FB82BB90h)
00007FF6FB813F80  test         al, al
00007FF6FB813F82  jne          static void Fabled_Engine::main+5DDFh (00007FF6FB81907Fh)
00007FF6FB813F88  movdqu       xmm0, xmmword ptr [rsp+100h]
00007FF6FB813F91  movdqa       xmmword ptr [rsp+40h], xmm0
00007FF6FB813F97  mov          rbx, qword ptr [rsp+110h]
00007FF6FB813F9F  mov          qword ptr [rsp+50h], rbx
00007FF6FB813FA4  mov          r14, qword ptr [rsp+40h]
00007FF6FB813FA9  cmp          rbx, 8h
00007FF6FB813FAD  mov          rdi, 517CC1B727220A95h
00007FF6FB813FB7  jb           static void Fabled_Engine::main+D40h (00007FF6FB813FE0h)
00007FF6FB813FB9  lea          rax, [rbx-8h]
00007FF6FB813FBD  mov          rdx, rax
00007FF6FB813FC0  shr          rdx, 3h
00007FF6FB813FC4  add          rdx, 1h
00007FF6FB813FC8  mov          ebp, edx
00007FF6FB813FCA  and          ebp, 3h
00007FF6FB813FCD  cmp          rax, 18h
00007FF6FB813FD1  jae          static void Fabled_Engine::main+D50h (00007FF6FB813FF0h)
00007FF6FB813FD3  mov          rax, r14
00007FF6FB813FD6  xor          r15d, r15d
00007FF6FB813FD9  jmp          static void Fabled_Engine::main+D9Dh (00007FF6FB81403Dh)
00007FF6FB813FDB  nop          dword ptr [rax+rax*1], eax
00007FF6FB813FE0  xor          r15d, r15d
00007FF6FB813FE3  mov          rax, r14
00007FF6FB813FE6  jmp          static void Fabled_Engine::main+DCBh (00007FF6FB81406Bh)
00007FF6FB813FEB  nop          dword ptr [rax+rax*1], eax
00007FF6FB813FF0  and          rdx, FFFFFFFFFFFFFFFCh
00007FF6FB813FF4  neg          rdx
00007FF6FB813FF7  mov          rax, r14
00007FF6FB813FFA  xor          r15d, r15d
00007FF6FB813FFD  nop          dword ptr [rax], eax
00007FF6FB814000  rol          r15, 5h
00007FF6FB814004  xor          r15, qword ptr [rax]
00007FF6FB814007  imul         r15, rdi
00007FF6FB81400B  rol          r15, 5h
00007FF6FB81400F  xor          r15, qword ptr [rax+8h]
00007FF6FB814013  imul         r15, rdi
00007FF6FB814017  rol          r15, 5h
00007FF6FB81401B  xor          r15, qword ptr [rax+10h]
00007FF6FB81401F  imul         r15, rdi
00007FF6FB814023  rol          r15, 5h
00007FF6FB814027  xor          r15, qword ptr [rax+18h]
00007FF6FB81402B  imul         r15, rdi
00007FF6FB81402F  add          rax, 20h
00007FF6FB814033  add          rbx, FFFFFFFFFFFFFFE0h
00007FF6FB814037  add          rdx, 4h
00007FF6FB81403B  jne          static void Fabled_Engine::main+D60h (00007FF6FB814000h)
00007FF6FB81403D  test         rbp, rbp
00007FF6FB814040  je           static void Fabled_Engine::main+DCBh (00007FF6FB81406Bh)
00007FF6FB814042  shl          rbp, 3h
00007FF6FB814046  xor          ecx, ecx
00007FF6FB814048  nop          dword ptr [rax+rax*1], eax
00007FF6FB814050  rol          r15, 5h
00007FF6FB814054  xor          r15, qword ptr [rax+rcx*1]
00007FF6FB814058  imul         r15, rdi
00007FF6FB81405C  add          rcx, 8h
00007FF6FB814060  cmp          rbp, rcx
00007FF6FB814063  jne          static void Fabled_Engine::main+DB0h (00007FF6FB814050h)
00007FF6FB814065  add          rax, rcx
00007FF6FB814068  sub          rbx, rcx
00007FF6FB81406B  cmp          rbx, 3h
00007FF6FB81406F  jbe          static void Fabled_Engine::main+DE6h (00007FF6FB814086h)
00007FF6FB814071  mov          ecx, dword ptr [rax]
00007FF6FB814073  rol          r15, 5h
00007FF6FB814077  xor          r15, rcx
00007FF6FB81407A  imul         r15, rdi
00007FF6FB81407E  add          rax, 4h
00007FF6FB814082  add          rbx, FFFFFFFFFFFFFFFCh
00007FF6FB814086  test         rbx, rbx
00007FF6FB814089  mov          qword ptr [rsp+28h], rsi
00007FF6FB81408E  je           static void Fabled_Engine::main+E84h (00007FF6FB814124h)
00007FF6FB814094  lea          rbp, [rbx-1h]
00007FF6FB814098  mov          rcx, rbx
00007FF6FB81409B  mov          rdx, rax
00007FF6FB81409E  and          rcx, 3h
00007FF6FB8140A2  je           static void Fabled_Engine::main+E28h (00007FF6FB8140C8h)
00007FF6FB8140A4  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB8140AE  nop
00007FF6FB8140B0  movzx        esi, byte ptr [rdx]
00007FF6FB8140B3  rol          r15, 5h
00007FF6FB8140B7  add          rdx, 1h
00007FF6FB8140BB  xor          r15, rsi
00007FF6FB8140BE  imul         r15, rdi
00007FF6FB8140C2  add          rcx, FFFFFFFFFFFFFFFFh
00007FF6FB8140C6  jne          static void Fabled_Engine::main+E10h (00007FF6FB8140B0h)
00007FF6FB8140C8  cmp          rbp, 3h
00007FF6FB8140CC  jb           static void Fabled_Engine::main+E84h (00007FF6FB814124h)
00007FF6FB8140CE  add          rax, rbx
00007FF6FB8140D1  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB8140DB  nop          dword ptr [rax+rax*1], eax
00007FF6FB8140E0  rol          r15, 5h
00007FF6FB8140E4  movzx        ecx, byte ptr [rdx]
00007FF6FB8140E7  xor          r15, rcx
00007FF6FB8140EA  imul         r15, rdi
00007FF6FB8140EE  movzx        ecx, byte ptr [rdx+1h]
00007FF6FB8140F2  rol          r15, 5h
00007FF6FB8140F6  xor          r15, rcx
00007FF6FB8140F9  imul         r15, rdi
00007FF6FB8140FD  movzx        ecx, byte ptr [rdx+2h]
00007FF6FB814101  rol          r15, 5h
00007FF6FB814105  xor          r15, rcx
00007FF6FB814108  imul         r15, rdi
00007FF6FB81410C  movzx        ecx, byte ptr [rdx+3h]
00007FF6FB814110  rol          r15, 5h
00007FF6FB814114  xor          r15, rcx
00007FF6FB814117  imul         r15, rdi
00007FF6FB81411B  add          rdx, 4h
00007FF6FB81411F  cmp          rdx, rax
00007FF6FB814122  jne          static void Fabled_Engine::main+E40h (00007FF6FB8140E0h)
00007FF6FB814124  add          qword ptr [rsp+28h], 10h
00007FF6FB81412A  rol          r15, 5h
00007FF6FB81412E  xor          r15, FFh
00007FF6FB814135  imul         r15, rdi
00007FF6FB814139  lea          rcx, [rsp+1E0h]
00007FF6FB814141  mov          rdx, r15
00007FF6FB814144  lea          r8, [rsp+40h]
00007FF6FB814149  call         static union enum$<core::option::Option<hashbrown::raw::Bucket<tuple<alloc::string::String, tuple<>>>>, 1, 18446744073709551615, Some> hashbrown::raw::RawTable<tuple<alloc::string::String, tuple<>>, alloc::alloc::Global>::find<tuple<alloc::string::String, tuple<>>,alloc::alloc::Global,closure-0> (00007FF6FB845460h)
00007FF6FB81414E  test         rax, rax
00007FF6FB814151  je           static void Fabled_Engine::main+EE0h (00007FF6FB814180h)
00007FF6FB814153  cmp          qword ptr [rsp+48h], 0h
00007FF6FB814159  je           static void Fabled_Engine::main+C48h (00007FF6FB813EE8h)
00007FF6FB81415F  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB814166  xor          edx, edx
00007FF6FB814168  mov          r8, r14
00007FF6FB81416B  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB814170  jmp          static void Fabled_Engine::main+C48h (00007FF6FB813EE8h)
00007FF6FB814175  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB81417F  nop
00007FF6FB814180  mov          rbp, qword ptr [rsp+1E0h]
00007FF6FB814188  mov          rax, qword ptr [rsp+1E8h]
00007FF6FB814190  mov          rcx, rbp
00007FF6FB814193  and          rcx, r15
00007FF6FB814196  movdqu       xmm0, xmmword ptr [rax+rcx*1]
00007FF6FB81419B  pmovmskb     ebx, xmm0
00007FF6FB81419F  test         bx, bx
00007FF6FB8141A2  jne          static void Fabled_Engine::main+F28h (00007FF6FB8141C8h)
00007FF6FB8141A4  mov          edx, 10h
00007FF6FB8141A9  nop          dword ptr [rax], eax
00007FF6FB8141B0  add          rcx, rdx
00007FF6FB8141B3  and          rcx, rbp
00007FF6FB8141B6  movdqu       xmm0, xmmword ptr [rax+rcx*1]
00007FF6FB8141BB  pmovmskb     ebx, xmm0
00007FF6FB8141BF  add          rdx, 10h
00007FF6FB8141C3  test         bx, bx
00007FF6FB8141C6  je           static void Fabled_Engine::main+F10h (00007FF6FB8141B0h)
00007FF6FB8141C8  bsf          dx, bx
00007FF6FB8141CC  movzx        edx, dx
00007FF6FB8141CF  add          rdx, rcx
00007FF6FB8141D2  and          rdx, rbp
00007FF6FB8141D5  mov          bl, byte ptr [rax+rdx*1]
00007FF6FB8141D8  test         bl, bl
00007FF6FB8141DA  jns          static void Fabled_Engine::main+FBBh (00007FF6FB81425Bh)
00007FF6FB8141DC  and          bl, 1h
00007FF6FB8141DF  cmp          qword ptr [rsp+1F0h], 0h
00007FF6FB8141E8  jne          static void Fabled_Engine::main+C00h (00007FF6FB813EA0h)
00007FF6FB8141EE  test         bl, bl
00007FF6FB8141F0  je           static void Fabled_Engine::main+C00h (00007FF6FB813EA0h)
00007FF6FB8141F6  mov          r8d, 1h
00007FF6FB8141FC  lea          rcx, [rsp+2C0h]
00007FF6FB814204  lea          rdx, [rsp+1E0h]
00007FF6FB81420C  call         static union enum$<core::result::Result<tuple<>, enum$<hashbrown::TryReserveError, 1, 18446744073709551615, AllocError>>> hashbrown::raw::RawTable<tuple<alloc::string::String, tuple<>>, alloc::alloc::Global>::reserve_rehash<tuple<alloc::string::String, tuple<>>,alloc::alloc::Global,closure-0> (00007FF6FB927F70h)
00007FF6FB814211  mov          rbp, qword ptr [rsp+1E0h]
00007FF6FB814219  mov          rax, qword ptr [rsp+1E8h]
00007FF6FB814221  mov          rcx, rbp
00007FF6FB814224  and          rcx, r15
00007FF6FB814227  movdqu       xmm0, xmmword ptr [rax+rcx*1]
00007FF6FB81422C  pmovmskb     esi, xmm0
00007FF6FB814230  test         si, si
00007FF6FB814233  jne          static void Fabled_Engine::main+BDEh (00007FF6FB813E7Eh)
00007FF6FB814239  mov          edx, 10h
00007FF6FB81423E  add          rcx, rdx
00007FF6FB814241  and          rcx, rbp
00007FF6FB814244  movdqu       xmm0, xmmword ptr [rax+rcx*1]
00007FF6FB814249  pmovmskb     esi, xmm0
00007FF6FB81424D  add          rdx, 10h
00007FF6FB814251  test         si, si
00007FF6FB814254  je           static void Fabled_Engine::main+F9Eh (00007FF6FB81423Eh)
00007FF6FB814256  jmp          static void Fabled_Engine::main+BDEh (00007FF6FB813E7Eh)
00007FF6FB81425B  movdqa       xmm0, xmmword ptr [rax]
00007FF6FB81425F  pmovmskb     ecx, xmm0
00007FF6FB814263  bsf          cx, cx
00007FF6FB814267  movzx        edx, cx
00007FF6FB81426A  mov          bl, byte ptr [rax+rdx*1]
00007FF6FB81426D  and          bl, 1h
00007FF6FB814270  cmp          qword ptr [rsp+1F0h], 0h
00007FF6FB814279  je           static void Fabled_Engine::main+F4Eh (00007FF6FB8141EEh)
00007FF6FB81427F  jmp          static void Fabled_Engine::main+C00h (00007FF6FB813EA0h)
00007FF6FB814284  movdqa       xmm0, xmmword ptr [rax]
00007FF6FB814288  pmovmskb     ecx, xmm0
00007FF6FB81428C  bsf          cx, cx
00007FF6FB814290  movzx        edx, cx
00007FF6FB814293  jmp          static void Fabled_Engine::main+C00h (00007FF6FB813EA0h)
00007FF6FB814298  nop          dword ptr [rax+rax*1], eax
00007FF6FB8142A0  lea          rax, [rsp+698h]
00007FF6FB8142A8  movups       xmmword ptr [rax], xmm8
00007FF6FB8142AC  mov          rax, qword ptr [00007FF6FB945F50h]
00007FF6FB8142B3  mov          qword ptr [rsp+690h], rax
00007FF6FB8142BB  mov          rax, qword ptr [rsp+A78h]
00007FF6FB8142C3  test         rax, rax
00007FF6FB8142C6  je           static void Fabled_Engine::main+1420h (00007FF6FB8146C0h)
00007FF6FB8142CC  mov          r14, qword ptr [rsp+A68h]
00007FF6FB8142D4  lea          rsi, [rax*8-8h]
00007FF6FB8142DC  shr          rsi, 3h
00007FF6FB8142E0  xor          edi, edi
00007FF6FB8142E2  mov          qword ptr [rsp+38h], rsi
00007FF6FB8142E7  jmp          static void Fabled_Engine::main+107Eh (00007FF6FB81431Eh)
00007FF6FB8142E9  nop          dword ptr [rax], eax
00007FF6FB8142F0  add          dword ptr [rsp+218h], FFFFFFFFh
00007FF6FB8142F8  lea          rbx, [rsp+2C0h]
00007FF6FB814300  mov          rsi, qword ptr [rsp+38h]
00007FF6FB814305  mov          rdi, qword ptr [rsp+28h]
00007FF6FB81430A  add          r14, 38h
00007FF6FB81430E  lea          rax, [rdi+1h]
00007FF6FB814312  cmp          rdi, rsi
00007FF6FB814315  mov          rdi, rax
00007FF6FB814318  je           static void Fabled_Engine::main+1420h (00007FF6FB8146C0h)
00007FF6FB81431E  mov          eax, FFFFFFFFh
00007FF6FB814323  cmp          rdi, rax
00007FF6FB814326  je           static void Fabled_Engine::main+1420h (00007FF6FB8146C0h)
00007FF6FB81432C  lea          r15d, [rdi+1h]
00007FF6FB814330  mov          r8, qword ptr [r14]
00007FF6FB814333  test         r8, r8
00007FF6FB814336  je           static void Fabled_Engine::main+10A0h (00007FF6FB814340h)
00007FF6FB814338  mov          r9, qword ptr [r14+10h]
00007FF6FB81433C  jmp          static void Fabled_Engine::main+10ADh (00007FF6FB81434Dh)
00007FF6FB81433E  nop
00007FF6FB814340  mov          r9d, 4h
00007FF6FB814346  lea          r8, [00007FF6FB93B3D4h]
00007FF6FB81434D  lea          rcx, [rsp+D0h]
00007FF6FB814355  lea          rdx, [rsp+1C0h]
00007FF6FB81435D  call         static struct alloc::string::String naga::proc::namer::Namer::call (00007FF6FB8A3090h)
00007FF6FB814362  shl          r15, 20h
00007FF6FB814366  lea          r8, [r15+2h]
00007FF6FB81436A  mov          rax, qword ptr [rsp+E0h]
00007FF6FB814372  mov          qword ptr [rsp+2D0h], rax
00007FF6FB81437A  movdqu       xmm0, xmmword ptr [rsp+D0h]
00007FF6FB814383  movdqa       xmmword ptr [rsp+2C0h], xmm0
00007FF6FB81438C  mov          qword ptr [rsp+20h], rbx
00007FF6FB814391  lea          rcx, [rsp+40h]
00007FF6FB814396  lea          rdx, [rsp+620h]
00007FF6FB81439E  xor          r9d, r9d
00007FF6FB8143A1  call         static union enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some> hashbrown::map::HashMap<enum$<naga::proc::namer::NameKey>, alloc::string::String, core::hash::BuildHasherDefault<fxhash::FxHasher>, alloc::alloc::Global>::insert<enum$<naga::proc::namer::NameKey>,alloc::string::String,core::hash::BuildHasherDefault<fxhash::FxHasher>,alloc::alloc::Global> (00007FF6FB84CE60h)
00007FF6FB8143A6  mov          r8, qword ptr [rsp+40h]
00007FF6FB8143AB  test         r8, r8
00007FF6FB8143AE  je           static void Fabled_Engine::main+1126h (00007FF6FB8143C6h)
00007FF6FB8143B0  cmp          qword ptr [rsp+48h], 0h
00007FF6FB8143B6  je           static void Fabled_Engine::main+1126h (00007FF6FB8143C6h)
00007FF6FB8143B8  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB8143BF  xor          edx, edx
00007FF6FB8143C1  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB8143C6  cmp          byte ptr [r14+18h], 6h
00007FF6FB8143CB  jne          static void Fabled_Engine::main+106Ah (00007FF6FB81430Ah)
00007FF6FB8143D1  mov          qword ptr [rsp+28h], rdi
00007FF6FB8143D6  mov          r11d, dword ptr [rsp+218h]
00007FF6FB8143DE  add          r11d, 1h
00007FF6FB8143E2  mov          dword ptr [rsp+218h], r11d
00007FF6FB8143EA  mov          rax, qword ptr [r14+30h]
00007FF6FB8143EE  test         rax, rax
00007FF6FB8143F1  je           static void Fabled_Engine::main+1248h (00007FF6FB8144E8h)
00007FF6FB8143F7  mov          rbp, qword ptr [r14+20h]
00007FF6FB8143FB  shl          rax, 3h
00007FF6FB8143FF  lea          rdi, [rax+rax*4]
00007FF6FB814403  xor          ebx, ebx
00007FF6FB814405  xor          esi, esi
00007FF6FB814407  jmp          static void Fabled_Engine::main+1181h (00007FF6FB814421h)
00007FF6FB814409  nop          dword ptr [rax], eax
00007FF6FB814410  add          rsi, 1h
00007FF6FB814414  add          rbx, 28h
00007FF6FB814418  cmp          rdi, rbx
00007FF6FB81441B  je           static void Fabled_Engine::main+1240h (00007FF6FB8144E0h)
00007FF6FB814421  mov          r8, qword ptr [rbp+rbx*1]
00007FF6FB814426  test         r8, r8
00007FF6FB814429  je           static void Fabled_Engine::main+11A0h (00007FF6FB814440h)
00007FF6FB81442B  mov          r9, qword ptr [rbp+rbx*1+10h]
00007FF6FB814430  jmp          static void Fabled_Engine::main+11ADh (00007FF6FB81444Dh)
00007FF6FB814432  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB81443C  nop          dword ptr [rax], eax
00007FF6FB814440  mov          r9d, 6h
00007FF6FB814446  lea          r8, [00007FF6FB93B3D8h]
00007FF6FB81444D  lea          rcx, [rsp+100h]
00007FF6FB814455  lea          rdx, [rsp+1C0h]
00007FF6FB81445D  call         static struct alloc::string::String naga::proc::namer::Namer::call (00007FF6FB8A3090h)
00007FF6FB814462  mov          r8, r15
00007FF6FB814465  or           r8, 3h
00007FF6FB814469  mov          rax, qword ptr [rsp+110h]
00007FF6FB814471  mov          qword ptr [rsp+2D0h], rax
00007FF6FB814479  movdqu       xmm0, xmmword ptr [rsp+100h]
00007FF6FB814482  movdqa       xmmword ptr [rsp+2C0h], xmm0
00007FF6FB81448B  lea          rax, [rsp+2C0h]
00007FF6FB814493  mov          qword ptr [rsp+20h], rax
00007FF6FB814498  lea          rcx, [rsp+40h]
00007FF6FB81449D  lea          rdx, [rsp+620h]
00007FF6FB8144A5  mov          r9, rsi
00007FF6FB8144A8  call         static union enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some> hashbrown::map::HashMap<enum$<naga::proc::namer::NameKey>, alloc::string::String, core::hash::BuildHasherDefault<fxhash::FxHasher>, alloc::alloc::Global>::insert<enum$<naga::proc::namer::NameKey>,alloc::string::String,core::hash::BuildHasherDefault<fxhash::FxHasher>,alloc::alloc::Global> (00007FF6FB84CE60h)
00007FF6FB8144AD  mov          r8, qword ptr [rsp+40h]
00007FF6FB8144B2  test         r8, r8
00007FF6FB8144B5  je           static void Fabled_Engine::main+1170h (00007FF6FB814410h)
00007FF6FB8144BB  cmp          qword ptr [rsp+48h], 0h
00007FF6FB8144C1  je           static void Fabled_Engine::main+1170h (00007FF6FB814410h)
00007FF6FB8144C7  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB8144CE  xor          edx, edx
00007FF6FB8144D0  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB8144D5  jmp          static void Fabled_Engine::main+1170h (00007FF6FB814410h)
00007FF6FB8144DA  nop          word ptr [rax+rax*1], ax
00007FF6FB8144E0  mov          r11d, dword ptr [rsp+218h]
00007FF6FB8144E8  mov          rax, qword ptr [rsp+1C0h]
00007FF6FB8144F0  mov          rbx, qword ptr [rsp+1C8h]
00007FF6FB8144F8  lea          r15, [rax+rbx*1]
00007FF6FB8144FC  add          r15, 1h
00007FF6FB814500  movdqa       xmm0, xmmword ptr [rbx]
00007FF6FB814504  pmovmskb     ecx, xmm0
00007FF6FB814508  lea          rsi, [rbx+10h]
00007FF6FB81450C  cmp          cx, FFFFh
00007FF6FB814510  je           static void Fabled_Engine::main+1290h (00007FF6FB814530h)
00007FF6FB814512  mov          eax, ecx
00007FF6FB814514  not          eax
00007FF6FB814516  mov          edx, FFFFFFFEh
00007FF6FB81451B  sub          edx, ecx
00007FF6FB81451D  mov          ebp, eax
00007FF6FB81451F  and          ebp, edx
00007FF6FB814521  jmp          static void Fabled_Engine::main+12BFh (00007FF6FB81455Fh)
00007FF6FB814523  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB81452D  nop          dword ptr [rax], eax
00007FF6FB814530  cmp          rsi, r15
00007FF6FB814533  jae          static void Fabled_Engine::main+1050h (00007FF6FB8142F0h)
00007FF6FB814539  movdqa       xmm0, xmmword ptr [rsi]
00007FF6FB81453D  pmovmskb     eax, xmm0
00007FF6FB814541  add          rbx, FFFFFFFFFFFFFD80h
00007FF6FB814548  add          rsi, 10h
00007FF6FB81454C  cmp          ax, FFFFh
00007FF6FB814550  je           static void Fabled_Engine::main+1290h (00007FF6FB814530h)
00007FF6FB814552  mov          ecx, FFFFFFFEh
00007FF6FB814557  sub          ecx, eax
00007FF6FB814559  not          eax
00007FF6FB81455B  mov          ebp, eax
00007FF6FB81455D  and          ebp, ecx
00007FF6FB81455F  bsf          ax, ax
00007FF6FB814563  movzx        eax, ax
00007FF6FB814566  neg          rax
00007FF6FB814569  lea          rax, [rax+rax*4]
00007FF6FB81456D  lea          rax, [rbx+rax*8]
00007FF6FB814571  cmp          dword ptr [rax-10h], r11d
00007FF6FB814575  jne          static void Fabled_Engine::main+13ABh (00007FF6FB81464Bh)
00007FF6FB81457B  mov          rcx, qword ptr [rsp+1C8h]
00007FF6FB814583  mov          rdx, rcx
00007FF6FB814586  sub          rdx, rax
00007FF6FB814589  sar          rdx, 3h
00007FF6FB81458D  mov          rdi, CCCCCCCCCCCCCCCDh
00007FF6FB814597  imul         rdx, rdi
00007FF6FB81459B  lea          r8, [rdx-10h]
00007FF6FB81459F  and          r8, qword ptr [rsp+1C0h]
00007FF6FB8145A7  movdqu       xmm0, xmmword ptr [rcx+r8*1]
00007FF6FB8145AD  pcmpeqb      xmm0, xmm9
00007FF6FB8145B2  pmovmskb     edi, xmm0
00007FF6FB8145B6  mov          r9w, 10h
00007FF6FB8145BB  mov          r10w, 10h
00007FF6FB8145C0  test         di, di
00007FF6FB8145C3  je           static void Fabled_Engine::main+132Eh (00007FF6FB8145CEh)
00007FF6FB8145C5  bsr          r10w, di
00007FF6FB8145CA  xor          r10d, Fh
00007FF6FB8145CE  movdqu       xmm0, xmmword ptr [rcx+rdx*1]
00007FF6FB8145D3  pcmpeqb      xmm0, xmm9
00007FF6FB8145D8  pmovmskb     edi, xmm0
00007FF6FB8145DC  test         di, di
00007FF6FB8145DF  je           static void Fabled_Engine::main+1346h (00007FF6FB8145E6h)
00007FF6FB8145E1  bsf          r9w, di
00007FF6FB8145E6  movzx        edi, r9w
00007FF6FB8145EA  add          edi, r10d
00007FF6FB8145ED  cmp          di, Fh
00007FF6FB8145F1  jbe          static void Fabled_Engine::main+1370h (00007FF6FB814610h)
00007FF6FB8145F3  mov          byte ptr [rcx+rdx*1], 80h
00007FF6FB8145F7  mov          byte ptr [r8+rcx*1+10h], 80h
00007FF6FB8145FD  add          qword ptr [rsp+1D8h], FFFFFFFFFFFFFFFFh
00007FF6FB814606  cmp          qword ptr [rax-20h], 0h
00007FF6FB81460B  jne          static void Fabled_Engine::main+1393h (00007FF6FB814633h)
00007FF6FB81460D  jmp          static void Fabled_Engine::main+13ABh (00007FF6FB81464Bh)
00007FF6FB81460F  nop
00007FF6FB814610  add          qword ptr [rsp+1D0h], 1h
00007FF6FB814619  mov          byte ptr [rcx+rdx*1], FFh
00007FF6FB81461D  mov          byte ptr [r8+rcx*1+10h], FFh
00007FF6FB814623  add          qword ptr [rsp+1D8h], FFFFFFFFFFFFFFFFh
00007FF6FB81462C  cmp          qword ptr [rax-20h], 0h
00007FF6FB814631  je           static void Fabled_Engine::main+13ABh (00007FF6FB81464Bh)
00007FF6FB814633  mov          r8, qword ptr [rax-28h]
00007FF6FB814637  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB81463E  xor          edx, edx
00007FF6FB814640  mov          edi, r11d
00007FF6FB814643  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB814648  mov          r11d, edi
00007FF6FB81464B  test         bp, bp
00007FF6FB81464E  je           static void Fabled_Engine::main+13C0h (00007FF6FB814660h)
00007FF6FB814650  lea          ecx, [rbp-1h]
00007FF6FB814653  and          ecx, ebp
00007FF6FB814655  jmp          static void Fabled_Engine::main+13EFh (00007FF6FB81468Fh)
00007FF6FB814657  nop          word ptr [rax+rax*1], ax
00007FF6FB814660  cmp          rsi, r15
00007FF6FB814663  jae          static void Fabled_Engine::main+1050h (00007FF6FB8142F0h)
00007FF6FB814669  movdqa       xmm0, xmmword ptr [rsi]
00007FF6FB81466D  pmovmskb     ebp, xmm0
00007FF6FB814671  add          rbx, FFFFFFFFFFFFFD80h
00007FF6FB814678  add          rsi, 10h
00007FF6FB81467C  cmp          bp, FFFFh
00007FF6FB814680  je           static void Fabled_Engine::main+13C0h (00007FF6FB814660h)
00007FF6FB814682  mov          eax, FFFFFFFEh
00007FF6FB814687  sub          eax, ebp
00007FF6FB814689  not          ebp
00007FF6FB81468B  mov          ecx, ebp
00007FF6FB81468D  and          ecx, eax
00007FF6FB81468F  bsf          ax, bp
00007FF6FB814693  movzx        eax, ax
00007FF6FB814696  neg          rax
00007FF6FB814699  lea          rax, [rax+rax*4]
00007FF6FB81469D  lea          rax, [rbx+rax*8]
00007FF6FB8146A1  mov          ebp, ecx
00007FF6FB8146A3  test         rbx, rbx
00007FF6FB8146A6  jne          static void Fabled_Engine::main+12D1h (00007FF6FB814571h)
00007FF6FB8146AC  jmp          static void Fabled_Engine::main+1050h (00007FF6FB8142F0h)
00007FF6FB8146B1  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB8146BB  nop          dword ptr [rax+rax*1], eax
00007FF6FB8146C0  mov          rax, qword ptr [rsp+AD8h]
00007FF6FB8146C8  test         rax, rax
00007FF6FB8146CB  je           static void Fabled_Engine::main+16F0h (00007FF6FB814990h)
00007FF6FB8146D1  mov          rsi, qword ptr [rsp+AC8h]
00007FF6FB8146D9  imul         rax, rax, D0h
00007FF6FB8146E0  add          rax, rsi
00007FF6FB8146E3  mov          qword ptr [rsp+38h], rax
00007FF6FB8146E8  xor          r15d, r15d
00007FF6FB8146EB  jmp          static void Fabled_Engine::main+1464h (00007FF6FB814704h)
00007FF6FB8146ED  nop          dword ptr [rax], eax
00007FF6FB8146F0  mov          r15, qword ptr [rsp+28h]
00007FF6FB8146F5  add          r15, 1h
00007FF6FB8146F9  cmp          rsi, qword ptr [rsp+38h]
00007FF6FB8146FE  je           static void Fabled_Engine::main+16F0h (00007FF6FB814990h)
00007FF6FB814704  mov          r14, rsi
00007FF6FB814707  mov          r8, qword ptr [rsi]
00007FF6FB81470A  mov          r9, qword ptr [rsi+10h]
00007FF6FB81470E  lea          rcx, [rsp+D0h]
00007FF6FB814716  lea          rdx, [rsp+1C0h]
00007FF6FB81471E  call         static struct alloc::string::String naga::proc::namer::Namer::call (00007FF6FB8A3090h)
00007FF6FB814723  mov          qword ptr [rsp+28h], r15
00007FF6FB814728  shl          r15d, 10h
00007FF6FB81472C  lea          r8, [r15+7h]
00007FF6FB814730  mov          rax, qword ptr [rsp+E0h]
00007FF6FB814738  mov          qword ptr [rsp+2D0h], rax
00007FF6FB814740  movdqu       xmm0, xmmword ptr [rsp+D0h]
00007FF6FB814749  movdqa       xmmword ptr [rsp+2C0h], xmm0
00007FF6FB814752  mov          qword ptr [rsp+20h], rbx
00007FF6FB814757  lea          rcx, [rsp+40h]
00007FF6FB81475C  lea          rdx, [rsp+620h]
00007FF6FB814764  xor          r9d, r9d
00007FF6FB814767  call         static union enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some> hashbrown::map::HashMap<enum$<naga::proc::namer::NameKey>, alloc::string::String, core::hash::BuildHasherDefault<fxhash::FxHasher>, alloc::alloc::Global>::insert<enum$<naga::proc::namer::NameKey>,alloc::string::String,core::hash::BuildHasherDefault<fxhash::FxHasher>,alloc::alloc::Global> (00007FF6FB84CE60h)
00007FF6FB81476C  mov          r8, qword ptr [rsp+40h]
00007FF6FB814771  test         r8, r8
00007FF6FB814774  je           static void Fabled_Engine::main+14ECh (00007FF6FB81478Ch)
00007FF6FB814776  cmp          qword ptr [rsp+48h], 0h
00007FF6FB81477C  je           static void Fabled_Engine::main+14ECh (00007FF6FB81478Ch)
00007FF6FB81477E  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB814785  xor          edx, edx
00007FF6FB814787  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB81478C  mov          rax, qword ptr [r14+40h]
00007FF6FB814790  test         rax, rax
00007FF6FB814793  je           static void Fabled_Engine::main+15E0h (00007FF6FB814880h)
00007FF6FB814799  mov          rsi, qword ptr [r14+30h]
00007FF6FB81479D  shl          rax, 3h
00007FF6FB8147A1  lea          rdi, [rax+rax*4]
00007FF6FB8147A5  xor          ebp, ebp
00007FF6FB8147A7  xor          ebx, ebx
00007FF6FB8147A9  jmp          static void Fabled_Engine::main+152Ah (00007FF6FB8147CAh)
00007FF6FB8147AB  nop          dword ptr [rax+rax*1], eax
00007FF6FB8147B0  mov          rax, 100000000h
00007FF6FB8147BA  add          rbx, rax
00007FF6FB8147BD  add          rbp, 28h
00007FF6FB8147C1  cmp          rdi, rbp
00007FF6FB8147C4  je           static void Fabled_Engine::main+15E0h (00007FF6FB814880h)
00007FF6FB8147CA  mov          r8, qword ptr [rsi+rbp*1]
00007FF6FB8147CE  test         r8, r8
00007FF6FB8147D1  je           static void Fabled_Engine::main+1540h (00007FF6FB8147E0h)
00007FF6FB8147D3  mov          r9, qword ptr [rsi+rbp*1+10h]
00007FF6FB8147D8  jmp          static void Fabled_Engine::main+154Dh (00007FF6FB8147EDh)
00007FF6FB8147DA  nop          word ptr [rax+rax*1], ax
00007FF6FB8147E0  mov          r9d, 5h
00007FF6FB8147E6  lea          r8, [00007FF6FB93B3DEh]
00007FF6FB8147ED  lea          rcx, [rsp+100h]
00007FF6FB8147F5  lea          rdx, [rsp+1C0h]
00007FF6FB8147FD  call         static struct alloc::string::String naga::proc::namer::Namer::call (00007FF6FB8A3090h)
00007FF6FB814802  mov          r8, r15
00007FF6FB814805  or           r8, rbx
00007FF6FB814808  or           r8, 9h
00007FF6FB81480C  mov          rax, qword ptr [rsp+110h]
00007FF6FB814814  mov          qword ptr [rsp+2D0h], rax
00007FF6FB81481C  movdqu       xmm0, xmmword ptr [rsp+100h]
00007FF6FB814825  movdqa       xmmword ptr [rsp+2C0h], xmm0
00007FF6FB81482E  lea          rax, [rsp+2C0h]
00007FF6FB814836  mov          qword ptr [rsp+20h], rax
00007FF6FB81483B  lea          rcx, [rsp+40h]
00007FF6FB814840  lea          rdx, [rsp+620h]
00007FF6FB814848  xor          r9d, r9d
00007FF6FB81484B  call         static union enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some> hashbrown::map::HashMap<enum$<naga::proc::namer::NameKey>, alloc::string::String, core::hash::BuildHasherDefault<fxhash::FxHasher>, alloc::alloc::Global>::insert<enum$<naga::proc::namer::NameKey>,alloc::string::String,core::hash::BuildHasherDefault<fxhash::FxHasher>,alloc::alloc::Global> (00007FF6FB84CE60h)
00007FF6FB814850  mov          r8, qword ptr [rsp+40h]
00007FF6FB814855  test         r8, r8
00007FF6FB814858  je           static void Fabled_Engine::main+1510h (00007FF6FB8147B0h)
00007FF6FB81485E  cmp          qword ptr [rsp+48h], 0h
00007FF6FB814864  je           static void Fabled_Engine::main+1510h (00007FF6FB8147B0h)
00007FF6FB81486A  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB814871  xor          edx, edx
00007FF6FB814873  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB814878  jmp          static void Fabled_Engine::main+1510h (00007FF6FB8147B0h)
00007FF6FB81487D  nop          dword ptr [rax], eax
00007FF6FB814880  lea          rsi, [r14+D0h]
00007FF6FB814887  mov          rdi, qword ptr [r14+58h]
00007FF6FB81488B  test         rdi, rdi
00007FF6FB81488E  lea          rbx, [rsp+2C0h]
00007FF6FB814896  je           static void Fabled_Engine::main+1450h (00007FF6FB8146F0h)
00007FF6FB81489C  mov          rbp, qword ptr [r14+48h]
00007FF6FB8148A0  shl          rdi, 5h
00007FF6FB8148A4  add          rdi, FFFFFFFFFFFFFFE0h
00007FF6FB8148A8  shr          rdi, 5h
00007FF6FB8148AC  add          rdi, 1h
00007FF6FB8148B0  add          rbp, 10h
00007FF6FB8148B4  xor          r14d, r14d
00007FF6FB8148B7  jmp          static void Fabled_Engine::main+162Dh (00007FF6FB8148CDh)
00007FF6FB8148B9  nop          dword ptr [rax], eax
00007FF6FB8148C0  add          rbp, 20h
00007FF6FB8148C4  cmp          rdi, r14
00007FF6FB8148C7  je           static void Fabled_Engine::main+1450h (00007FF6FB8146F0h)
00007FF6FB8148CD  mov          eax, FFFFFFFFh
00007FF6FB8148D2  cmp          r14, rax
00007FF6FB8148D5  je           static void Fabled_Engine::main+1450h (00007FF6FB8146F0h)
00007FF6FB8148DB  add          r14, 1h
00007FF6FB8148DF  mov          r8, qword ptr [rbp-10h]
00007FF6FB8148E3  test         r8, r8
00007FF6FB8148E6  je           static void Fabled_Engine::main+1650h (00007FF6FB8148F0h)
00007FF6FB8148E8  mov          r9, qword ptr [rbp]
00007FF6FB8148EC  jmp          static void Fabled_Engine::main+165Dh (00007FF6FB8148FDh)
00007FF6FB8148EE  nop
00007FF6FB8148F0  mov          r9d, 5h
00007FF6FB8148F6  lea          r8, [00007FF6FB93B3E3h]
00007FF6FB8148FD  lea          rcx, [rsp+100h]
00007FF6FB814905  lea          rdx, [rsp+1C0h]
00007FF6FB81490D  call         static struct alloc::string::String naga::proc::namer::Namer::call (00007FF6FB8A3090h)
00007FF6FB814912  mov          r8, r14
00007FF6FB814915  shl          r8, 20h
00007FF6FB814919  or           r8, r15
00007FF6FB81491C  or           r8, 8h
00007FF6FB814920  mov          rax, qword ptr [rsp+110h]
00007FF6FB814928  mov          qword ptr [rsp+2D0h], rax
00007FF6FB814930  movdqu       xmm0, xmmword ptr [rsp+100h]
00007FF6FB814939  movdqa       xmmword ptr [rsp+2C0h], xmm0
00007FF6FB814942  mov          qword ptr [rsp+20h], rbx
00007FF6FB814947  lea          rcx, [rsp+40h]
00007FF6FB81494C  lea          rdx, [rsp+620h]
00007FF6FB814954  xor          r9d, r9d
00007FF6FB814957  call         static union enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some> hashbrown::map::HashMap<enum$<naga::proc::namer::NameKey>, alloc::string::String, core::hash::BuildHasherDefault<fxhash::FxHasher>, alloc::alloc::Global>::insert<enum$<naga::proc::namer::NameKey>,alloc::string::String,core::hash::BuildHasherDefault<fxhash::FxHasher>,alloc::alloc::Global> (00007FF6FB84CE60h)
00007FF6FB81495C  mov          r8, qword ptr [rsp+40h]
00007FF6FB814961  test         r8, r8
00007FF6FB814964  je           static void Fabled_Engine::main+1620h (00007FF6FB8148C0h)
00007FF6FB81496A  cmp          qword ptr [rsp+48h], 0h
00007FF6FB814970  je           static void Fabled_Engine::main+1620h (00007FF6FB8148C0h)
00007FF6FB814976  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB81497D  xor          edx, edx
00007FF6FB81497F  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB814984  jmp          static void Fabled_Engine::main+1620h (00007FF6FB8148C0h)
00007FF6FB814989  nop          dword ptr [rax], eax
00007FF6FB814990  mov          rax, qword ptr [rsp+AC0h]
00007FF6FB814998  test         rax, rax
00007FF6FB81499B  je           static void Fabled_Engine::main+1A00h (00007FF6FB814CA0h)
00007FF6FB8149A1  mov          rdi, qword ptr [rsp+AB0h]
00007FF6FB8149A9  lea          rax, [rax*8-8h]
00007FF6FB8149B1  shr          rax, 3h
00007FF6FB8149B5  mov          qword ptr [rsp+38h], rax
00007FF6FB8149BA  xor          ecx, ecx
00007FF6FB8149BC  jmp          static void Fabled_Engine::main+173Eh (00007FF6FB8149DEh)
00007FF6FB8149BE  nop
00007FF6FB8149C0  add          rdi, A8h
00007FF6FB8149C7  mov          rcx, qword ptr [rsp+28h]
00007FF6FB8149CC  lea          rax, [rcx+1h]
00007FF6FB8149D0  cmp          rcx, qword ptr [rsp+38h]
00007FF6FB8149D5  mov          rcx, rax
00007FF6FB8149D8  je           static void Fabled_Engine::main+1A00h (00007FF6FB814CA0h)
00007FF6FB8149DE  mov          eax, FFFFFFFFh
00007FF6FB8149E3  cmp          rcx, rax
00007FF6FB8149E6  je           static void Fabled_Engine::main+1A00h (00007FF6FB814CA0h)
00007FF6FB8149EC  mov          qword ptr [rsp+28h], rcx
00007FF6FB8149F1  lea          r14d, [rcx+1h]
00007FF6FB8149F5  mov          r8, qword ptr [rdi]
00007FF6FB8149F8  test         r8, r8
00007FF6FB8149FB  je           static void Fabled_Engine::main+1770h (00007FF6FB814A10h)
00007FF6FB8149FD  mov          r9, qword ptr [rdi+10h]
00007FF6FB814A01  jmp          static void Fabled_Engine::main+177Dh (00007FF6FB814A1Dh)
00007FF6FB814A03  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB814A0D  nop          dword ptr [rax], eax
00007FF6FB814A10  mov          r9d, 8h
00007FF6FB814A16  lea          r8, [00007FF6FB93B3E8h]
00007FF6FB814A1D  lea          rcx, [rsp+D0h]
00007FF6FB814A25  lea          rdx, [rsp+1C0h]
00007FF6FB814A2D  call         static struct alloc::string::String naga::proc::namer::Namer::call (00007FF6FB8A3090h)
00007FF6FB814A32  shl          r14, 20h
00007FF6FB814A36  lea          r8, [r14+4h]
00007FF6FB814A3A  mov          rax, qword ptr [rsp+E0h]
00007FF6FB814A42  mov          qword ptr [rsp+2D0h], rax
00007FF6FB814A4A  movdqu       xmm0, xmmword ptr [rsp+D0h]
00007FF6FB814A53  movdqa       xmmword ptr [rsp+2C0h], xmm0
00007FF6FB814A5C  mov          qword ptr [rsp+20h], rbx
00007FF6FB814A61  lea          rcx, [rsp+40h]
00007FF6FB814A66  lea          rdx, [rsp+620h]
00007FF6FB814A6E  xor          r9d, r9d
00007FF6FB814A71  call         static union enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some> hashbrown::map::HashMap<enum$<naga::proc::namer::NameKey>, alloc::string::String, core::hash::BuildHasherDefault<fxhash::FxHasher>, alloc::alloc::Global>::insert<enum$<naga::proc::namer::NameKey>,alloc::string::String,core::hash::BuildHasherDefault<fxhash::FxHasher>,alloc::alloc::Global> (00007FF6FB84CE60h)
00007FF6FB814A76  mov          r8, qword ptr [rsp+40h]
00007FF6FB814A7B  test         r8, r8
00007FF6FB814A7E  je           static void Fabled_Engine::main+17F6h (00007FF6FB814A96h)
00007FF6FB814A80  cmp          qword ptr [rsp+48h], 0h
00007FF6FB814A86  je           static void Fabled_Engine::main+17F6h (00007FF6FB814A96h)
00007FF6FB814A88  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB814A8F  xor          edx, edx
00007FF6FB814A91  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB814A96  mov          r15, rdi
00007FF6FB814A99  mov          rax, qword ptr [rdi+28h]
00007FF6FB814A9D  test         rax, rax
00007FF6FB814AA0  je           static void Fabled_Engine::main+18F0h (00007FF6FB814B90h)
00007FF6FB814AA6  mov          rbp, qword ptr [r15+18h]
00007FF6FB814AAA  shl          rax, 3h
00007FF6FB814AAE  lea          rdi, [rax+rax*4]
00007FF6FB814AB2  xor          ebx, ebx
00007FF6FB814AB4  xor          esi, esi
00007FF6FB814AB6  jmp          static void Fabled_Engine::main+1831h (00007FF6FB814AD1h)
00007FF6FB814AB8  nop          dword ptr [rax+rax*1], eax
00007FF6FB814AC0  add          rsi, 1h
00007FF6FB814AC4  add          rbx, 28h
00007FF6FB814AC8  cmp          rdi, rbx
00007FF6FB814ACB  je           static void Fabled_Engine::main+18F0h (00007FF6FB814B90h)
00007FF6FB814AD1  mov          r8, qword ptr [rbp+rbx*1]
00007FF6FB814AD6  test         r8, r8
00007FF6FB814AD9  je           static void Fabled_Engine::main+1850h (00007FF6FB814AF0h)
00007FF6FB814ADB  mov          r9, qword ptr [rbp+rbx*1+10h]
00007FF6FB814AE0  jmp          static void Fabled_Engine::main+185Dh (00007FF6FB814AFDh)
00007FF6FB814AE2  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB814AEC  nop          dword ptr [rax], eax
00007FF6FB814AF0  mov          r9d, 5h
00007FF6FB814AF6  lea          r8, [00007FF6FB93B3DEh]
00007FF6FB814AFD  lea          rcx, [rsp+100h]
00007FF6FB814B05  lea          rdx, [rsp+1C0h]
00007FF6FB814B0D  call         static struct alloc::string::String naga::proc::namer::Namer::call (00007FF6FB8A3090h)
00007FF6FB814B12  mov          r8, r14
00007FF6FB814B15  or           r8, 5h
00007FF6FB814B19  mov          rax, qword ptr [rsp+110h]
00007FF6FB814B21  mov          qword ptr [rsp+2D0h], rax
00007FF6FB814B29  movdqu       xmm0, xmmword ptr [rsp+100h]
00007FF6FB814B32  movdqa       xmmword ptr [rsp+2C0h], xmm0
00007FF6FB814B3B  lea          rax, [rsp+2C0h]
00007FF6FB814B43  mov          qword ptr [rsp+20h], rax
00007FF6FB814B48  lea          rcx, [rsp+40h]
00007FF6FB814B4D  lea          rdx, [rsp+620h]
00007FF6FB814B55  mov          r9, rsi
00007FF6FB814B58  call         static union enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some> hashbrown::map::HashMap<enum$<naga::proc::namer::NameKey>, alloc::string::String, core::hash::BuildHasherDefault<fxhash::FxHasher>, alloc::alloc::Global>::insert<enum$<naga::proc::namer::NameKey>,alloc::string::String,core::hash::BuildHasherDefault<fxhash::FxHasher>,alloc::alloc::Global> (00007FF6FB84CE60h)
00007FF6FB814B5D  mov          r8, qword ptr [rsp+40h]
00007FF6FB814B62  test         r8, r8
00007FF6FB814B65  je           static void Fabled_Engine::main+1820h (00007FF6FB814AC0h)
00007FF6FB814B6B  cmp          qword ptr [rsp+48h], 0h
00007FF6FB814B71  je           static void Fabled_Engine::main+1820h (00007FF6FB814AC0h)
00007FF6FB814B77  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB814B7E  xor          edx, edx
00007FF6FB814B80  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB814B85  jmp          static void Fabled_Engine::main+1820h (00007FF6FB814AC0h)
00007FF6FB814B8A  nop          word ptr [rax+rax*1], ax
00007FF6FB814B90  mov          rdi, r15
00007FF6FB814B93  mov          r15, qword ptr [r15+40h]
00007FF6FB814B97  test         r15, r15
00007FF6FB814B9A  lea          rbx, [rsp+2C0h]
00007FF6FB814BA2  je           static void Fabled_Engine::main+1720h (00007FF6FB8149C0h)
00007FF6FB814BA8  mov          rbp, qword ptr [rdi+30h]
00007FF6FB814BAC  shl          r15, 5h
00007FF6FB814BB0  add          r15, FFFFFFFFFFFFFFE0h
00007FF6FB814BB4  shr          r15, 5h
00007FF6FB814BB8  add          r15, 1h
00007FF6FB814BBC  add          rbp, 10h
00007FF6FB814BC0  xor          esi, esi
00007FF6FB814BC2  jmp          static void Fabled_Engine::main+193Dh (00007FF6FB814BDDh)
00007FF6FB814BC4  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB814BCE  nop
00007FF6FB814BD0  add          rbp, 20h
00007FF6FB814BD4  cmp          r15, rsi
00007FF6FB814BD7  je           static void Fabled_Engine::main+1720h (00007FF6FB8149C0h)
00007FF6FB814BDD  mov          eax, FFFFFFFFh
00007FF6FB814BE2  cmp          rsi, rax
00007FF6FB814BE5  je           static void Fabled_Engine::main+1720h (00007FF6FB8149C0h)
00007FF6FB814BEB  add          rsi, 1h
00007FF6FB814BEF  mov          r8, qword ptr [rbp-10h]
00007FF6FB814BF3  test         r8, r8
00007FF6FB814BF6  je           static void Fabled_Engine::main+1960h (00007FF6FB814C00h)
00007FF6FB814BF8  mov          r9, qword ptr [rbp]
00007FF6FB814BFC  jmp          static void Fabled_Engine::main+196Dh (00007FF6FB814C0Dh)
00007FF6FB814BFE  nop
00007FF6FB814C00  mov          r9d, 5h
00007FF6FB814C06  lea          r8, [00007FF6FB93B3E3h]
00007FF6FB814C0D  lea          rcx, [rsp+100h]
00007FF6FB814C15  lea          rdx, [rsp+1C0h]
00007FF6FB814C1D  call         static struct alloc::string::String naga::proc::namer::Namer::call (00007FF6FB8A3090h)
00007FF6FB814C22  mov          r8, r14
00007FF6FB814C25  or           r8, 6h
00007FF6FB814C29  mov          rax, qword ptr [rsp+110h]
00007FF6FB814C31  mov          qword ptr [rsp+2D0h], rax
00007FF6FB814C39  movdqu       xmm0, xmmword ptr [rsp+100h]
00007FF6FB814C42  movdqa       xmmword ptr [rsp+2C0h], xmm0
00007FF6FB814C4B  mov          qword ptr [rsp+20h], rbx
00007FF6FB814C50  lea          rcx, [rsp+40h]
00007FF6FB814C55  lea          rdx, [rsp+620h]
00007FF6FB814C5D  mov          r9, rsi
00007FF6FB814C60  call         static union enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some> hashbrown::map::HashMap<enum$<naga::proc::namer::NameKey>, alloc::string::String, core::hash::BuildHasherDefault<fxhash::FxHasher>, alloc::alloc::Global>::insert<enum$<naga::proc::namer::NameKey>,alloc::string::String,core::hash::BuildHasherDefault<fxhash::FxHasher>,alloc::alloc::Global> (00007FF6FB84CE60h)
00007FF6FB814C65  mov          r8, qword ptr [rsp+40h]
00007FF6FB814C6A  test         r8, r8
00007FF6FB814C6D  je           static void Fabled_Engine::main+1930h (00007FF6FB814BD0h)
00007FF6FB814C73  cmp          qword ptr [rsp+48h], 0h
00007FF6FB814C79  je           static void Fabled_Engine::main+1930h (00007FF6FB814BD0h)
00007FF6FB814C7F  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB814C86  xor          edx, edx
00007FF6FB814C88  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB814C8D  jmp          static void Fabled_Engine::main+1930h (00007FF6FB814BD0h)
00007FF6FB814C92  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB814C9C  nop          dword ptr [rax], eax
00007FF6FB814CA0  mov          rax, qword ptr [rsp+AA8h]
00007FF6FB814CA8  test         rax, rax
00007FF6FB814CAB  lea          r14, [rsp+690h]
00007FF6FB814CB3  je           static void Fabled_Engine::main+1B10h (00007FF6FB814DB0h)
00007FF6FB814CB9  mov          rsi, qword ptr [rsp+A98h]
00007FF6FB814CC1  lea          rdi, [rax*8-8h]
00007FF6FB814CC9  shr          rdi, 3h
00007FF6FB814CCD  add          rdi, 1h
00007FF6FB814CD1  add          rsi, 10h
00007FF6FB814CD5  xor          ebx, ebx
00007FF6FB814CD7  jmp          static void Fabled_Engine::main+1A4Dh (00007FF6FB814CEDh)
00007FF6FB814CD9  nop          dword ptr [rax], eax
00007FF6FB814CE0  add          rsi, 38h
00007FF6FB814CE4  cmp          rdi, rbx
00007FF6FB814CE7  je           static void Fabled_Engine::main+1B10h (00007FF6FB814DB0h)
00007FF6FB814CED  mov          eax, FFFFFFFFh
00007FF6FB814CF2  cmp          rbx, rax
00007FF6FB814CF5  je           static void Fabled_Engine::main+1B10h (00007FF6FB814DB0h)
00007FF6FB814CFB  add          rbx, 1h
00007FF6FB814CFF  mov          r8, qword ptr [rsi-10h]
00007FF6FB814D03  test         r8, r8
00007FF6FB814D06  je           static void Fabled_Engine::main+1A70h (00007FF6FB814D10h)
00007FF6FB814D08  mov          r9, qword ptr [rsi]
00007FF6FB814D0B  jmp          static void Fabled_Engine::main+1A7Dh (00007FF6FB814D1Dh)
00007FF6FB814D0D  nop          dword ptr [rax], eax
00007FF6FB814D10  mov          r9d, 6h
00007FF6FB814D16  lea          r8, [00007FF6FB93B3F0h]
00007FF6FB814D1D  lea          rcx, [rsp+100h]
00007FF6FB814D25  lea          rdx, [rsp+1C0h]
00007FF6FB814D2D  call         static struct alloc::string::String naga::proc::namer::Namer::call (00007FF6FB8A3090h)
00007FF6FB814D32  mov          r8, rbx
00007FF6FB814D35  shl          r8, 20h
00007FF6FB814D39  or           r8, 1h
00007FF6FB814D3D  mov          rax, qword ptr [rsp+110h]
00007FF6FB814D45  mov          qword ptr [rsp+2D0h], rax
00007FF6FB814D4D  movdqu       xmm0, xmmword ptr [rsp+100h]
00007FF6FB814D56  movdqa       xmmword ptr [rsp+2C0h], xmm0
00007FF6FB814D5F  lea          rax, [rsp+2C0h]
00007FF6FB814D67  mov          qword ptr [rsp+20h], rax
00007FF6FB814D6C  lea          rcx, [rsp+40h]
00007FF6FB814D71  lea          rdx, [rsp+620h]
00007FF6FB814D79  xor          r9d, r9d
00007FF6FB814D7C  call         static union enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some> hashbrown::map::HashMap<enum$<naga::proc::namer::NameKey>, alloc::string::String, core::hash::BuildHasherDefault<fxhash::FxHasher>, alloc::alloc::Global>::insert<enum$<naga::proc::namer::NameKey>,alloc::string::String,core::hash::BuildHasherDefault<fxhash::FxHasher>,alloc::alloc::Global> (00007FF6FB84CE60h)
00007FF6FB814D81  mov          r8, qword ptr [rsp+40h]
00007FF6FB814D86  test         r8, r8
00007FF6FB814D89  je           static void Fabled_Engine::main+1A40h (00007FF6FB814CE0h)
00007FF6FB814D8F  cmp          qword ptr [rsp+48h], 0h
00007FF6FB814D95  je           static void Fabled_Engine::main+1A40h (00007FF6FB814CE0h)
00007FF6FB814D9B  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB814DA2  xor          edx, edx
00007FF6FB814DA4  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB814DA9  jmp          static void Fabled_Engine::main+1A40h (00007FF6FB814CE0h)
00007FF6FB814DAE  nop
00007FF6FB814DB0  mov          rdi, qword ptr [rsp+A90h]
00007FF6FB814DB8  test         rdi, rdi
00007FF6FB814DBB  je           static void Fabled_Engine::main+2090h (00007FF6FB815330h)
00007FF6FB814DC1  mov          rbx, qword ptr [rsp+A80h]
00007FF6FB814DC9  shl          rdi, 6h
00007FF6FB814DCD  add          rdi, FFFFFFFFFFFFFFC0h
00007FF6FB814DD1  shr          rdi, 6h
00007FF6FB814DD5  xor          ebp, ebp
00007FF6FB814DD7  jmp          static void Fabled_Engine::main+1B54h (00007FF6FB814DF4h)
00007FF6FB814DD9  nop          dword ptr [rax], eax
00007FF6FB814DE0  add          rbx, 40h
00007FF6FB814DE4  lea          rax, [rbp+1h]
00007FF6FB814DE8  cmp          rbp, rdi
00007FF6FB814DEB  mov          rbp, rax
00007FF6FB814DEE  je           static void Fabled_Engine::main+2090h (00007FF6FB815330h)
00007FF6FB814DF4  mov          eax, FFFFFFFFh
00007FF6FB814DF9  cmp          rbp, rax
00007FF6FB814DFC  je           static void Fabled_Engine::main+2090h (00007FF6FB815330h)
00007FF6FB814E02  cmp          qword ptr [rbx], 0h
00007FF6FB814E06  je           static void Fabled_Engine::main+1B80h (00007FF6FB814E20h)
00007FF6FB814E08  lea          rax, [rbx+10h]
00007FF6FB814E0C  mov          rcx, rbx
00007FF6FB814E0F  jmp          static void Fabled_Engine::main+1F73h (00007FF6FB815213h)
00007FF6FB814E14  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB814E1E  nop
00007FF6FB814E20  mov          qword ptr [rsp+6A0h], 0h
00007FF6FB814E2C  cmp          byte ptr [rbx+18h], 1h
00007FF6FB814E30  jne          static void Fabled_Engine::main+1C84h (00007FF6FB814F24h)
00007FF6FB814E36  mov          r15d, dword ptr [rbx+1Ch]
00007FF6FB814E3A  mov          dword ptr [rsp+104h], r15d
00007FF6FB814E42  mov          word ptr [rsp+100h], 2h
00007FF6FB814E4C  mov          qword ptr [rsp+2C0h], 0h
00007FF6FB814E58  lea          rcx, [rsp+100h]
00007FF6FB814E60  lea          rdx, [rsp+2C0h]
00007FF6FB814E68  call         static void naga::proc::namer::{{impl}}::hash<fxhash::FxHasher> (00007FF6FB8A50A0h)
00007FF6FB814E6D  mov          rax, qword ptr [rsp+2C0h]
00007FF6FB814E75  mov          r9, qword ptr [rsp+620h]
00007FF6FB814E7D  mov          r10, qword ptr [rsp+628h]
00007FF6FB814E85  mov          rdx, r9
00007FF6FB814E88  and          rdx, rax
00007FF6FB814E8B  shr          rax, 39h
00007FF6FB814E8F  movdqu       xmm1, xmmword ptr [r10+rdx*1]
00007FF6FB814E95  movd         xmm0, eax
00007FF6FB814E99  punpcklbw    xmm0, xmm0
00007FF6FB814E9D  pshuflw      xmm0, xmm0, 0h
00007FF6FB814EA2  pshufd       xmm0, xmm0, 0h
00007FF6FB814EA7  movdqa       xmm2, xmm0
00007FF6FB814EAB  pcmpeqb      xmm2, xmm1
00007FF6FB814EAF  pmovmskb     eax, xmm2
00007FF6FB814EB3  lea          rsi, [r10-28h]
00007FF6FB814EB7  xor          r8d, r8d
00007FF6FB814EBA  test         ax, ax
00007FF6FB814EBD  je           static void Fabled_Engine::main+1C50h (00007FF6FB814EF0h)
00007FF6FB814EBF  mov          ecx, eax
00007FF6FB814EC1  lea          eax, [rcx-1h]
00007FF6FB814EC4  and          eax, ecx
00007FF6FB814EC6  bsf          cx, cx
00007FF6FB814ECA  movzx        ecx, cx
00007FF6FB814ECD  add          rcx, rdx
00007FF6FB814ED0  and          rcx, r9
00007FF6FB814ED3  neg          rcx
00007FF6FB814ED6  lea          rcx, [rcx+rcx*4]
00007FF6FB814EDA  cmp          word ptr [rsi+rcx*8], 2h
00007FF6FB814EDF  jne          static void Fabled_Engine::main+1C1Ah (00007FF6FB814EBAh)
00007FF6FB814EE1  cmp          r15d, dword ptr [rsi+rcx*8+4h]
00007FF6FB814EE6  je           static void Fabled_Engine::main+1CD1h (00007FF6FB814F71h)
00007FF6FB814EEC  jmp          static void Fabled_Engine::main+1C1Ah (00007FF6FB814EBAh)
00007FF6FB814EEE  nop
00007FF6FB814EF0  pcmpeqb      xmm1, xmm9
00007FF6FB814EF5  pmovmskb     eax, xmm1
00007FF6FB814EF9  test         ax, ax
00007FF6FB814EFC  jne          static void Fabled_Engine::main+2F12h (00007FF6FB8161B2h)
00007FF6FB814F02  add          rdx, r8
00007FF6FB814F05  add          rdx, 10h
00007FF6FB814F09  add          r8, 10h
00007FF6FB814F0D  and          rdx, r9
00007FF6FB814F10  movdqu       xmm1, xmmword ptr [r10+rdx*1]
00007FF6FB814F16  movdqa       xmm2, xmm0
00007FF6FB814F1A  pcmpeqb      xmm2, xmm1
00007FF6FB814F1E  pmovmskb     eax, xmm2
00007FF6FB814F22  jmp          static void Fabled_Engine::main+1C1Ah (00007FF6FB814EBAh)
00007FF6FB814F24  movzx        eax, byte ptr [rbx+20h]
00007FF6FB814F28  lea          rcx, [00007FF6FB819D00h]
00007FF6FB814F2F  movsxd       rax, dword ptr [rcx+rax*4]
00007FF6FB814F33  add          rax, rcx
00007FF6FB814F36  jmp          rax
00007FF6FB814F38  mov          rax, qword ptr [rbx+28h]
00007FF6FB814F3C  mov          qword ptr [rsp+D0h], rax
00007FF6FB814F44  lea          rax, [rsp+D0h]
00007FF6FB814F4C  mov          qword ptr [rsp+40h], rax
00007FF6FB814F51  lea          rax, [static void core::fmt::num::imp::{{impl}}::fmt (00007FF6FB835500h)]
00007FF6FB814F58  mov          qword ptr [rsp+48h], rax
00007FF6FB814F5D  mov          qword ptr [rsp+100h], r14
00007FF6FB814F65  lea          rax, [00007FF6FB93B458h]
00007FF6FB814F6C  jmp          static void Fabled_Engine::main+1F0Bh (00007FF6FB8151ABh)
00007FF6FB814F71  lea          rax, [rsi+rcx*8]
00007FF6FB814F75  add          rax, 10h
00007FF6FB814F79  mov          qword ptr [rsp+40h], rax
00007FF6FB814F7E  lea          rax, [static core::result::Result alloc::string::{{impl}}::fmt (00007FF6FB81ED00h)]
00007FF6FB814F85  mov          qword ptr [rsp+48h], rax
00007FF6FB814F8A  mov          qword ptr [rsp+D0h], r14
00007FF6FB814F92  lea          rax, [00007FF6FB93B410h]
00007FF6FB814F99  mov          qword ptr [rsp+2C0h], rax
00007FF6FB814FA1  mov          qword ptr [rsp+2C8h], 1h
00007FF6FB814FAD  mov          qword ptr [rsp+2D0h], 0h
00007FF6FB814FB9  lea          rax, [rsp+40h]
00007FF6FB814FBE  mov          qword ptr [rsp+2E0h], rax
00007FF6FB814FC6  mov          qword ptr [rsp+2E8h], 1h
00007FF6FB814FD2  lea          rcx, [rsp+D0h]
00007FF6FB814FDA  jmp          static void Fabled_Engine::main+1F4Ch (00007FF6FB8151ECh)
00007FF6FB814FDF  movsd        xmm6, qword ptr [rbx+28h]
00007FF6FB814FE4  xor          eax, eax
00007FF6FB814FE6  ucomisd      xmm11, xmm6
00007FF6FB814FEB  andpd        xmm6, xmm10
00007FF6FB814FF0  lea          rcx, [00007FF6FB949BD8h]
00007FF6FB814FF7  lea          rdx, [00007FF6FB93B498h]
00007FF6FB814FFE  cmova        rcx, rdx
00007FF6FB815002  seta         al
00007FF6FB815005  mov          qword ptr [rsp+100h], rcx
00007FF6FB81500D  mov          qword ptr [rsp+108h], rax
00007FF6FB815015  movapd       xmm0, xmm6
00007FF6FB815019  call         trunc (00007FF6FB91E6E9h)
00007FF6FB81501E  movapd       xmm7, xmm0
00007FF6FB815022  movsd        qword ptr [rsp+250h], xmm0
00007FF6FB81502B  lea          rax, [rsp+100h]
00007FF6FB815033  mov          qword ptr [rsp+40h], rax
00007FF6FB815038  lea          rax, [static core::result::Result core::fmt::{{impl}}::fmt<str> (00007FF6FB828820h)]
00007FF6FB81503F  mov          qword ptr [rsp+48h], rax
00007FF6FB815044  lea          rax, [rsp+250h]
00007FF6FB81504C  mov          qword ptr [rsp+50h], rax
00007FF6FB815051  lea          rax, [static void core::fmt::float::{{impl}}::fmt (00007FF6FB834F10h)]
00007FF6FB815058  mov          qword ptr [rsp+58h], rax
00007FF6FB81505D  mov          qword ptr [rsp+D0h], r14
00007FF6FB815065  lea          rax, [00007FF6FB93B4A0h]
00007FF6FB81506C  mov          qword ptr [rsp+2C0h], rax
00007FF6FB815074  mov          qword ptr [rsp+2C8h], 2h
00007FF6FB815080  mov          qword ptr [rsp+2D0h], 0h
00007FF6FB81508C  lea          rax, [rsp+40h]
00007FF6FB815091  mov          qword ptr [rsp+2E0h], rax
00007FF6FB815099  mov          qword ptr [rsp+2E8h], 2h
00007FF6FB8150A5  lea          rcx, [rsp+D0h]
00007FF6FB8150AD  lea          rdx, [00007FF6FB93BCB8h]
00007FF6FB8150B4  lea          r8, [rsp+2C0h]
00007FF6FB8150BC  call         static void core::fmt::write (00007FF6FB82C910h)
00007FF6FB8150C1  test         al, al
00007FF6FB8150C3  jne          static void Fabled_Engine::main+612Eh (00007FF6FB8193CEh)
00007FF6FB8150C9  subsd        xmm6, xmm7
00007FF6FB8150CD  ucomisd      xmm6, xmm11
00007FF6FB8150D2  jne          static void Fabled_Engine::main+2009h (00007FF6FB8152A9h)
00007FF6FB8150D8  jp           static void Fabled_Engine::main+2009h (00007FF6FB8152A9h)
00007FF6FB8150DE  mov          qword ptr [rsp+40h], r14
00007FF6FB8150E3  lea          rax, [00007FF6FB93B530h]
00007FF6FB8150EA  mov          qword ptr [rsp+2C0h], rax
00007FF6FB8150F2  mov          qword ptr [rsp+2C8h], 1h
00007FF6FB8150FE  mov          qword ptr [rsp+2D0h], 0h
00007FF6FB81510A  lea          rax, [00007FF6FB949BD8h]
00007FF6FB815111  mov          qword ptr [rsp+2E0h], rax
00007FF6FB815119  mov          qword ptr [rsp+2E8h], 0h
00007FF6FB815125  lea          rcx, [rsp+40h]
00007FF6FB81512A  jmp          static void Fabled_Engine::main+1F4Ch (00007FF6FB8151ECh)
00007FF6FB81512F  mov          al, byte ptr [rbx+21h]
00007FF6FB815132  mov          byte ptr [rsp+D0h], al
00007FF6FB815139  lea          rax, [rsp+D0h]
00007FF6FB815141  mov          qword ptr [rsp+40h], rax
00007FF6FB815146  lea          rax, [static void core::fmt::{{impl}}::fmt (00007FF6FB831CA0h)]
00007FF6FB81514D  mov          qword ptr [rsp+48h], rax
00007FF6FB815152  mov          qword ptr [rsp+100h], r14
00007FF6FB81515A  lea          rax, [00007FF6FB93B410h]
00007FF6FB815161  mov          qword ptr [rsp+2C0h], rax
00007FF6FB815169  mov          qword ptr [rsp+2C8h], 1h
00007FF6FB815175  jmp          static void Fabled_Engine::main+1F1Fh (00007FF6FB8151BFh)
00007FF6FB815177  mov          rax, qword ptr [rbx+28h]
00007FF6FB81517B  mov          qword ptr [rsp+D0h], rax
00007FF6FB815183  lea          rax, [rsp+D0h]
00007FF6FB81518B  mov          qword ptr [rsp+40h], rax
00007FF6FB815190  lea          rax, [static void core::fmt::num::imp::{{impl}}::fmt (00007FF6FB82A470h)]
00007FF6FB815197  mov          qword ptr [rsp+48h], rax
00007FF6FB81519C  mov          qword ptr [rsp+100h], r14
00007FF6FB8151A4  lea          rax, [00007FF6FB93B478h]
00007FF6FB8151AB  mov          qword ptr [rsp+2C0h], rax
00007FF6FB8151B3  mov          qword ptr [rsp+2C8h], 2h
00007FF6FB8151BF  mov          qword ptr [rsp+2D0h], 0h
00007FF6FB8151CB  lea          rax, [rsp+40h]
00007FF6FB8151D0  mov          qword ptr [rsp+2E0h], rax
00007FF6FB8151D8  mov          qword ptr [rsp+2E8h], 1h
00007FF6FB8151E4  lea          rcx, [rsp+100h]
00007FF6FB8151EC  lea          rdx, [00007FF6FB93BCB8h]
00007FF6FB8151F3  lea          r8, [rsp+2C0h]
00007FF6FB8151FB  call         static void core::fmt::write (00007FF6FB82C910h)
00007FF6FB815200  test         al, al
00007FF6FB815202  jne          static void Fabled_Engine::main+5FB6h (00007FF6FB819256h)
00007FF6FB815208  mov          rcx, r14
00007FF6FB81520B  lea          rax, [rsp+6A0h]
00007FF6FB815213  lea          esi, [rbp+1h]
00007FF6FB815216  mov          r8, qword ptr [rcx]
00007FF6FB815219  mov          r9, qword ptr [rax]
00007FF6FB81521C  lea          rcx, [rsp+100h]
00007FF6FB815224  lea          rdx, [rsp+1C0h]
00007FF6FB81522C  call         static struct alloc::string::String naga::proc::namer::Namer::call (00007FF6FB8A3090h)
00007FF6FB815231  shl          rsi, 20h
00007FF6FB815235  mov          rax, qword ptr [rsp+110h]
00007FF6FB81523D  mov          qword ptr [rsp+2D0h], rax
00007FF6FB815245  movdqu       xmm0, xmmword ptr [rsp+100h]
00007FF6FB81524E  movdqa       xmmword ptr [rsp+2C0h], xmm0
00007FF6FB815257  lea          rax, [rsp+2C0h]
00007FF6FB81525F  mov          qword ptr [rsp+20h], rax
00007FF6FB815264  lea          rcx, [rsp+40h]
00007FF6FB815269  lea          rdx, [rsp+620h]
00007FF6FB815271  mov          r8, rsi
00007FF6FB815274  xor          r9d, r9d
00007FF6FB815277  call         static union enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some> hashbrown::map::HashMap<enum$<naga::proc::namer::NameKey>, alloc::string::String, core::hash::BuildHasherDefault<fxhash::FxHasher>, alloc::alloc::Global>::insert<enum$<naga::proc::namer::NameKey>,alloc::string::String,core::hash::BuildHasherDefault<fxhash::FxHasher>,alloc::alloc::Global> (00007FF6FB84CE60h)
00007FF6FB81527C  mov          r8, qword ptr [rsp+40h]
00007FF6FB815281  test         r8, r8
00007FF6FB815284  je           static void Fabled_Engine::main+1B40h (00007FF6FB814DE0h)
00007FF6FB81528A  cmp          qword ptr [rsp+48h], 0h
00007FF6FB815290  je           static void Fabled_Engine::main+1B40h (00007FF6FB814DE0h)
00007FF6FB815296  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB81529D  xor          edx, edx
00007FF6FB81529F  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB8152A4  jmp          static void Fabled_Engine::main+1B40h (00007FF6FB814DE0h)
00007FF6FB8152A9  mulsd        xmm6, xmm12
00007FF6FB8152AE  movapd       xmm0, xmm13
00007FF6FB8152B3  maxsd        xmm0, xmm6
00007FF6FB8152B7  movapd       xmm1, xmm14
00007FF6FB8152BC  minsd        xmm1, xmm0
00007FF6FB8152C0  cvttsd2si    eax, xmm1
00007FF6FB8152C4  mov          byte ptr [rsp+D0h], al
00007FF6FB8152CB  lea          rax, [rsp+D0h]
00007FF6FB8152D3  mov          qword ptr [rsp+40h], rax
00007FF6FB8152D8  lea          rax, [static void core::fmt::num::imp::{{impl}}::fmt (00007FF6FB834FE0h)]
00007FF6FB8152DF  mov          qword ptr [rsp+48h], rax
00007FF6FB8152E4  mov          qword ptr [rsp+100h], r14
00007FF6FB8152EC  lea          rax, [00007FF6FB93B4D8h]
00007FF6FB8152F3  mov          qword ptr [rsp+2C0h], rax
00007FF6FB8152FB  mov          qword ptr [rsp+2C8h], 2h
00007FF6FB815307  lea          rax, [00007FF6FB93B4F8h]
00007FF6FB81530E  mov          qword ptr [rsp+2D0h], rax
00007FF6FB815316  mov          qword ptr [rsp+2D8h], 1h
00007FF6FB815322  jmp          static void Fabled_Engine::main+1F2Bh (00007FF6FB8151CBh)
00007FF6FB815327  nop          word ptr [rax+rax*1], ax
00007FF6FB815330  cmp          qword ptr [rsp+698h], 0h
00007FF6FB815339  je           static void Fabled_Engine::main+20B1h (00007FF6FB815351h)
00007FF6FB81533B  mov          r8, qword ptr [rsp+690h]
00007FF6FB815343  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB81534A  xor          edx, edx
00007FF6FB81534C  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB815351  movups       xmm0, xmmword ptr [rsp+210h]
00007FF6FB815359  movaps       xmmword ptr [rsp+90h], xmm0
00007FF6FB815361  movups       xmm0, xmmword ptr [rsp+200h]
00007FF6FB815369  movaps       xmmword ptr [rsp+80h], xmm0
00007FF6FB815371  movups       xmm0, xmmword ptr [rsp+1C0h]
00007FF6FB815379  movups       xmm1, xmmword ptr [rsp+1D0h]
00007FF6FB815381  movups       xmm2, xmmword ptr [rsp+1E0h]
00007FF6FB815389  movups       xmm3, xmmword ptr [rsp+1F0h]
00007FF6FB815391  movaps       xmmword ptr [rsp+70h], xmm3
00007FF6FB815396  movaps       xmmword ptr [rsp+60h], xmm2
00007FF6FB81539B  movaps       xmmword ptr [rsp+50h], xmm1
00007FF6FB8153A0  movaps       xmmword ptr [rsp+40h], xmm0
00007FF6FB8153A5  lea          rcx, [rsp+AC8h]
00007FF6FB8153AD  call         static struct slice<u8> alloc::vec::{{impl}}::deref<u8,alloc::alloc::Global> (00007FF6FB812DA0h)
00007FF6FB8153B2  cmp          rdx, r12
00007FF6FB8153B5  lea          rbx, [rsp+2C0h]
00007FF6FB8153BD  jbe          static void Fabled_Engine::main+5DCEh (00007FF6FB81906Eh)
00007FF6FB8153C3  mov          rcx, rax
00007FF6FB8153C6  add          rcx, r13
00007FF6FB8153C9  lea          rbp, [rsp+388h]
00007FF6FB8153D1  movups       xmmword ptr [rbp+10h], xmm8
00007FF6FB8153D6  lea          rdx, [rsp+A68h]
00007FF6FB8153DE  mov          qword ptr [rsp+2C0h], rdx
00007FF6FB8153E6  lea          rdx, [rsp+150h]
00007FF6FB8153EE  mov          qword ptr [rsp+2C8h], rdx
00007FF6FB8153F6  lea          rdx, [rsp+5F0h]
00007FF6FB8153FE  mov          qword ptr [rsp+2D0h], rdx
00007FF6FB815406  lea          rdx, [rsp+270h]
00007FF6FB81540E  mov          qword ptr [rsp+2D8h], rdx
00007FF6FB815416  mov          dword ptr [rsp+3A8h], 0h
00007FF6FB815421  movaps       xmm0, xmmword ptr [rsp+90h]
00007FF6FB815429  movups       xmmword ptr [rbp-58h], xmm0
00007FF6FB81542D  movaps       xmm0, xmmword ptr [rsp+80h]
00007FF6FB815435  movups       xmmword ptr [rbp-68h], xmm0
00007FF6FB815439  movaps       xmm0, xmmword ptr [rsp+40h]
00007FF6FB81543E  movaps       xmm1, xmmword ptr [rsp+50h]
00007FF6FB815443  movdqa       xmm2, xmmword ptr [rsp+60h]
00007FF6FB815449  movdqa       xmm3, xmmword ptr [rsp+70h]
00007FF6FB81544F  movdqu       xmmword ptr [rbp-78h], xmm3
00007FF6FB815454  movdqu       xmmword ptr [rbp-88h], xmm2
00007FF6FB81545C  movups       xmmword ptr [rbp-98h], xmm1
00007FF6FB815463  movups       xmmword ptr [rbp-A8h], xmm0
00007FF6FB81546A  movdqu       xmm0, xmmword ptr [rsp+620h]
00007FF6FB815473  movupd       xmm1, xmmword ptr [rsp+630h]
00007FF6FB81547C  lea          rdx, [rsp+340h]
00007FF6FB815484  movupd       xmmword ptr [rdx+10h], xmm1
00007FF6FB815489  movdqu       xmmword ptr [rdx], xmm0
00007FF6FB81548D  mov          qword ptr [rsp+360h], 0h
00007FF6FB815499  lea          rdx, [00007FF6FB934260h]
00007FF6FB8154A0  mov          qword ptr [rsp+368h], rdx
00007FF6FB8154A8  movups       xmmword ptr [rbp-18h], xmm8
00007FF6FB8154AD  mov          qword ptr [rsp+380h], rcx
00007FF6FB8154B5  mov          word ptr [rsp+3B0h], r12w
00007FF6FB8154BE  mov          dword ptr [rsp+3ACh], 0h
00007FF6FB8154C9  mov          qword ptr [rsp+388h], 0h
00007FF6FB8154D5  mov          qword ptr [rsp+390h], rdx
00007FF6FB8154DD  mov          cl, byte ptr [rax+r13*1+CDh]
00007FF6FB8154E5  cmp          cl, 4h
00007FF6FB8154E8  je           static void Fabled_Engine::main+2262h (00007FF6FB815502h)
00007FF6FB8154EA  xor          edx, edx
00007FF6FB8154EC  cmp          cl, 3h
00007FF6FB8154EF  setne        dl
00007FF6FB8154F2  shl          edx, 9h
00007FF6FB8154F5  or           edx, 100h
00007FF6FB8154FB  mov          dword ptr [rsp+3A8h], edx
00007FF6FB815502  lea          rcx, [rax+r13*1]
00007FF6FB815506  add          rcx, 30h
00007FF6FB81550A  call         static struct slice<u8> alloc::vec::{{impl}}::deref<u8,alloc::alloc::Global> (00007FF6FB812DA0h)
00007FF6FB81550F  test         rdx, rdx
00007FF6FB815512  mov          r14d, 80h
00007FF6FB815518  lea          r15, [00007FF6FB819D10h]
00007FF6FB81551F  je           static void Fabled_Engine::main+22B2h (00007FF6FB815552h)
00007FF6FB815521  mov          rsi, rax
00007FF6FB815524  add          rsi, 1Ch
00007FF6FB815528  shl          rdx, 3h
00007FF6FB81552C  lea          rdi, [rdx+rdx*4]
00007FF6FB815530  cmp          byte ptr [rsi], 2h
00007FF6FB815533  mov          edx, 0h
00007FF6FB815538  cmovne       rdx, rsi
00007FF6FB81553C  mov          r8d, dword ptr [rsi-4h]
00007FF6FB815540  mov          rcx, rbx
00007FF6FB815543  call         static void naga::back::glsl::Writer<mut alloc::string::String*>::varying_required_features<mut alloc::string::String*> (00007FF6FB81A010h)
00007FF6FB815548  add          rsi, 28h
00007FF6FB81554C  add          rdi, FFFFFFFFFFFFFFD8h
00007FF6FB815550  jne          static void Fabled_Engine::main+2290h (00007FF6FB815530h)
00007FF6FB815552  mov          rax, qword ptr [rsp+380h]
00007FF6FB81555A  mov          cl, byte ptr [rax+B4h]
00007FF6FB815560  cmp          cl, 3h
00007FF6FB815563  jne          static void Fabled_Engine::main+2300h (00007FF6FB8155A0h)
00007FF6FB815565  mov          r8, qword ptr [rsp+2D8h]
00007FF6FB81556D  cmp          byte ptr [r8+1Ch], 2h
00007FF6FB815572  jne          static void Fabled_Engine::main+2331h (00007FF6FB8155D1h)
00007FF6FB815574  or           byte ptr [rsp+3A8h], 80h
00007FF6FB81557C  mov          r11, qword ptr [rsp+2C0h]
00007FF6FB815584  mov          rax, qword ptr [r11+10h]
00007FF6FB815588  test         rax, rax
00007FF6FB81558B  jne          static void Fabled_Engine::main+2346h (00007FF6FB8155E6h)
00007FF6FB81558D  jmp          static void Fabled_Engine::main+24A0h (00007FF6FB815740h)
00007FF6FB815592  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB81559C  nop          dword ptr [rax], eax
00007FF6FB8155A0  lea          rbp, [rax+B4h]
00007FF6FB8155A7  cmp          cl, 2h
00007FF6FB8155AA  mov          edx, 0h
00007FF6FB8155AF  cmovne       rdx, rbp
00007FF6FB8155B3  mov          r8d, dword ptr [rax+B0h]
00007FF6FB8155BA  mov          rcx, rbx
00007FF6FB8155BD  call         static void naga::back::glsl::Writer<mut alloc::string::String*>::varying_required_features<mut alloc::string::String*> (00007FF6FB81A010h)
00007FF6FB8155C2  mov          r8, qword ptr [rsp+2D8h]
00007FF6FB8155CA  cmp          byte ptr [r8+1Ch], 2h
00007FF6FB8155CF  je           static void Fabled_Engine::main+22D4h (00007FF6FB815574h)
00007FF6FB8155D1  mov          r11, qword ptr [rsp+2C0h]
00007FF6FB8155D9  mov          rax, qword ptr [r11+10h]
00007FF6FB8155DD  test         rax, rax
00007FF6FB8155E0  je           static void Fabled_Engine::main+24A0h (00007FF6FB815740h)
00007FF6FB8155E6  mov          rbx, qword ptr [r11]
00007FF6FB8155E9  lea          rdi, [rax*8-8h]
00007FF6FB8155F1  shr          rdi, 3h
00007FF6FB8155F5  add          rdi, 1h
00007FF6FB8155F9  add          rbx, 1Dh
00007FF6FB8155FD  xor          esi, esi
00007FF6FB8155FF  jmp          static void Fabled_Engine::main+2386h (00007FF6FB815626h)
00007FF6FB815601  cmp          byte ptr [rbx-3h], 2h
00007FF6FB815605  jne          static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB815607  cmp          byte ptr [rbx-2h], 8h
00007FF6FB81560B  jne          static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB81560D  or           byte ptr [rsp+3A8h], 4h
00007FF6FB815615  add          rsi, 1h
00007FF6FB815619  add          rbx, 38h
00007FF6FB81561D  cmp          rdi, rsi
00007FF6FB815620  je           static void Fabled_Engine::main+24A0h (00007FF6FB815740h)
00007FF6FB815626  mov          eax, FFFFFFFFh
00007FF6FB81562B  cmp          rsi, rax
00007FF6FB81562E  je           static void Fabled_Engine::main+24A0h (00007FF6FB815740h)
00007FF6FB815634  movzx        eax, byte ptr [rbx-5h]
00007FF6FB815638  cmp          rax, 7h
00007FF6FB81563C  ja           static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB81563E  movsxd       rax, dword ptr [r15+rax*4]
00007FF6FB815642  add          rax, r15
00007FF6FB815645  jmp          rax
00007FF6FB815647  cmp          byte ptr [rbx-4h], 2h
00007FF6FB81564B  jne          static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB81564D  cmp          byte ptr [rbx-3h], 8h
00007FF6FB815651  je           static void Fabled_Engine::main+236Dh (00007FF6FB81560Dh)
00007FF6FB815653  jmp          static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB815655  mov          ecx, dword ptr [rbx-1h]
00007FF6FB815658  add          ecx, FFFFFFFFh
00007FF6FB81565B  mov          rdx, qword ptr [r11+10h]
00007FF6FB81565F  cmp          rdx, rcx
00007FF6FB815662  jbe          static void Fabled_Engine::main+5DC0h (00007FF6FB819060h)
00007FF6FB815668  mov          rax, qword ptr [r11]
00007FF6FB81566B  imul         rcx, rcx, 38h
00007FF6FB81566F  cmp          byte ptr [rax+rcx*1+18h], 5h
00007FF6FB815674  jne          static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB815676  or           byte ptr [rsp+3A8h], 2h
00007FF6FB81567E  jmp          static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB815680  movzx        ecx, byte ptr [rbx-4h]
00007FF6FB815684  movzx        r10d, byte ptr [rbx-3h]
00007FF6FB815689  movzx        eax, byte ptr [rbx-2h]
00007FF6FB81568D  movzx        edx, byte ptr [rbx-1h]
00007FF6FB815691  movzx        r9d, byte ptr [rbx]
00007FF6FB815695  cmp          cl, 3h
00007FF6FB815698  jne          static void Fabled_Engine::main+243Ch (00007FF6FB8156DCh)
00007FF6FB81569A  test         r10b, r10b
00007FF6FB81569D  je           static void Fabled_Engine::main+243Ch (00007FF6FB8156DCh)
00007FF6FB81569F  mov          ecx, dword ptr [rsp+3A8h]
00007FF6FB8156A6  or           ecx, 40h
00007FF6FB8156A9  mov          dword ptr [rsp+3A8h], ecx
00007FF6FB8156B0  cmp          al, 2h
00007FF6FB8156B2  jne          static void Fabled_Engine::main+2444h (00007FF6FB8156E4h)
00007FF6FB8156B4  cmp          dl, 19h
00007FF6FB8156B7  ja           static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB8156BD  movzx        eax, dl
00007FF6FB8156C0  mov          ecx, 3E1C7FFh
00007FF6FB8156C5  bt           rcx, rax
00007FF6FB8156C9  jae          static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB8156CF  or           byte ptr [rsp+3A8h], 8h
00007FF6FB8156D7  jmp          static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB8156DC  test         cl, cl
00007FF6FB8156DE  je           static void Fabled_Engine::main+2481h (00007FF6FB815721h)
00007FF6FB8156E0  cmp          al, 2h
00007FF6FB8156E2  je           static void Fabled_Engine::main+2414h (00007FF6FB8156B4h)
00007FF6FB8156E4  test         al, al
00007FF6FB8156E6  jne          static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB8156EC  test         r9b, 1h
00007FF6FB8156F0  je           static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB8156F6  mov          eax, dword ptr [rsp+3A8h]
00007FF6FB8156FD  mov          ecx, eax
00007FF6FB8156FF  or           ecx, 10h
00007FF6FB815702  mov          dword ptr [rsp+3A8h], ecx
00007FF6FB815709  test         r10b, r10b
00007FF6FB81570C  je           static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB815712  or           eax, 30h
00007FF6FB815715  mov          dword ptr [rsp+3A8h], eax
00007FF6FB81571C  jmp          static void Fabled_Engine::main+2375h (00007FF6FB815615h)
00007FF6FB815721  mov          ecx, dword ptr [rsp+3A8h]
00007FF6FB815728  mov          ebp, 400h
00007FF6FB81572D  or           ecx, ebp
00007FF6FB81572F  mov          dword ptr [rsp+3A8h], ecx
00007FF6FB815736  cmp          al, 2h
00007FF6FB815738  jne          static void Fabled_Engine::main+2444h (00007FF6FB8156E4h)
00007FF6FB81573A  jmp          static void Fabled_Engine::main+2414h (00007FF6FB8156B4h)
00007FF6FB81573F  nop
00007FF6FB815740  mov          rax, qword ptr [r11+40h]
00007FF6FB815744  test         rax, rax
00007FF6FB815747  lea          rbx, [rsp+2C0h]
00007FF6FB81574F  je           static void Fabled_Engine::main+2520h (00007FF6FB8157C0h)
00007FF6FB815751  mov          rcx, qword ptr [r11+30h]
00007FF6FB815755  lea          rdx, [rax*8-8h]
00007FF6FB81575D  shr          rdx, 3h
00007FF6FB815761  add          rdx, 1h
00007FF6FB815765  add          rcx, 30h
00007FF6FB815769  xor          ebp, ebp
00007FF6FB81576B  jmp          static void Fabled_Engine::main+24EEh (00007FF6FB81578Eh)
00007FF6FB81576D  nop          dword ptr [rax], eax
00007FF6FB815770  mov          eax, dword ptr [rsp+3A8h]
00007FF6FB815777  or           eax, 1h
00007FF6FB81577A  mov          dword ptr [rsp+3A8h], eax
00007FF6FB815781  add          rbp, 1h
00007FF6FB815785  add          rcx, 38h
00007FF6FB815789  cmp          rdx, rbp
00007FF6FB81578C  je           static void Fabled_Engine::main+2520h (00007FF6FB8157C0h)
00007FF6FB81578E  mov          eax, FFFFFFFFh
00007FF6FB815793  cmp          rbp, rax
00007FF6FB815796  je           static void Fabled_Engine::main+2520h (00007FF6FB8157C0h)
00007FF6FB815798  movzx        eax, byte ptr [rcx]
00007FF6FB81579B  cmp          rax, 2h
00007FF6FB81579F  je           static void Fabled_Engine::main+2510h (00007FF6FB8157B0h)
00007FF6FB8157A1  cmp          eax, 4h
00007FF6FB8157A4  je           static void Fabled_Engine::main+24D0h (00007FF6FB815770h)
00007FF6FB8157A6  cmp          eax, 6h
00007FF6FB8157A9  jne          static void Fabled_Engine::main+24E1h (00007FF6FB815781h)
00007FF6FB8157AB  jmp          static void Fabled_Engine::main+286Dh (00007FF6FB815B0Dh)
00007FF6FB8157B0  mov          eax, dword ptr [rsp+3A8h]
00007FF6FB8157B7  or           eax, r14d
00007FF6FB8157BA  jmp          static void Fabled_Engine::main+24DAh (00007FF6FB81577Ah)
00007FF6FB8157BC  nop          dword ptr [rax], eax
00007FF6FB8157C0  movzx        edx, word ptr [r8+18h]
00007FF6FB8157C5  movzx        eax, word ptr [r8+1Ah]
00007FF6FB8157CA  mov          edi, dword ptr [rsp+3A8h]
00007FF6FB8157D1  test         dil, dil
00007FF6FB8157D4  js           static void Fabled_Engine::main+2560h (00007FF6FB815800h)
00007FF6FB8157D6  xor          r12d, r12d
00007FF6FB8157D9  test         dil, 1h
00007FF6FB8157DD  mov          esi, dword ptr [rsp+C8h]
00007FF6FB8157E4  je           static void Fabled_Engine::main+25AEh (00007FF6FB81584Eh)
00007FF6FB8157E6  xor          r12d, r12d
00007FF6FB8157E9  test         dx, dx
00007FF6FB8157EC  je           static void Fabled_Engine::main+259Fh (00007FF6FB81583Fh)
00007FF6FB8157EE  movzx        ebp, ax
00007FF6FB8157F1  xor          r12d, r12d
00007FF6FB8157F4  cmp          ebp, 135h
00007FF6FB8157FA  jbe          static void Fabled_Engine::main+25AAh (00007FF6FB81584Ah)
00007FF6FB8157FC  jmp          static void Fabled_Engine::main+25AEh (00007FF6FB81584Eh)
00007FF6FB8157FE  nop
00007FF6FB815800  movzx        ebp, ax
00007FF6FB815803  test         dx, dx
00007FF6FB815806  mov          esi, dword ptr [rsp+C8h]
00007FF6FB81580D  je           static void Fabled_Engine::main+2588h (00007FF6FB815828h)
00007FF6FB81580F  xor          r12d, r12d
00007FF6FB815812  cmp          ebp, 136h
00007FF6FB815818  jae          static void Fabled_Engine::main+25AEh (00007FF6FB81584Eh)
00007FF6FB81581A  mov          r12d, 80h
00007FF6FB815820  test         dil, 1h
00007FF6FB815824  jne          static void Fabled_Engine::main+25AAh (00007FF6FB81584Ah)
00007FF6FB815826  jmp          static void Fabled_Engine::main+25AEh (00007FF6FB81584Eh)
00007FF6FB815828  xor          r12d, r12d
00007FF6FB81582B  cmp          ebp, 1A4h
00007FF6FB815831  jae          static void Fabled_Engine::main+25AEh (00007FF6FB81584Eh)
00007FF6FB815833  mov          r12d, 80h
00007FF6FB815839  test         dil, 1h
00007FF6FB81583D  je           static void Fabled_Engine::main+25AEh (00007FF6FB81584Eh)
00007FF6FB81583F  movzx        ebp, ax
00007FF6FB815842  cmp          ebp, 190h
00007FF6FB815848  jae          static void Fabled_Engine::main+25AEh (00007FF6FB81584Eh)
00007FF6FB81584A  or           r12d, 1h
00007FF6FB81584E  test         dil, 4h
00007FF6FB815852  je           static void Fabled_Engine::main+25D6h (00007FF6FB815876h)
00007FF6FB815854  movzx        ebp, ax
00007FF6FB815857  mov          ebx, r12d
00007FF6FB81585A  or           ebx, 4h
00007FF6FB81585D  cmp          ebp, 96h
00007FF6FB815863  cmovb        r12d, ebx
00007FF6FB815867  test         dx, dx
00007FF6FB81586A  cmovne       r12d, ebx
00007FF6FB81586E  lea          rbx, [rsp+2C0h]
00007FF6FB815876  test         dil, 40h
00007FF6FB81587A  je           static void Fabled_Engine::main+25FAh (00007FF6FB81589Ah)
00007FF6FB81587C  movzx        ebp, ax
00007FF6FB81587F  test         dx, dx
00007FF6FB815882  je           static void Fabled_Engine::main+25EEh (00007FF6FB81588Eh)
00007FF6FB815884  cmp          ebp, 135h
00007FF6FB81588A  jbe          static void Fabled_Engine::main+25F6h (00007FF6FB815896h)
00007FF6FB81588C  jmp          static void Fabled_Engine::main+2627h (00007FF6FB8158C7h)
00007FF6FB81588E  cmp          ebp, 82h
00007FF6FB815894  jae          static void Fabled_Engine::main+2612h (00007FF6FB8158B2h)
00007FF6FB815896  or           r12d, 40h
00007FF6FB81589A  test         dil, 10h
00007FF6FB81589E  je           static void Fabled_Engine::main+2627h (00007FF6FB8158C7h)
00007FF6FB8158A0  test         dx, dx
00007FF6FB8158A3  je           static void Fabled_Engine::main+2618h (00007FF6FB8158B8h)
00007FF6FB8158A5  movzx        ebp, ax
00007FF6FB8158A8  cmp          ebp, 12Bh
00007FF6FB8158AE  jbe          static void Fabled_Engine::main+2623h (00007FF6FB8158C3h)
00007FF6FB8158B0  jmp          static void Fabled_Engine::main+2627h (00007FF6FB8158C7h)
00007FF6FB8158B2  test         dil, 10h
00007FF6FB8158B6  je           static void Fabled_Engine::main+2627h (00007FF6FB8158C7h)
00007FF6FB8158B8  movzx        ebp, ax
00007FF6FB8158BB  cmp          ebp, 96h
00007FF6FB8158C1  jae          static void Fabled_Engine::main+2627h (00007FF6FB8158C7h)
00007FF6FB8158C3  or           r12d, 10h
00007FF6FB8158C7  test         dil, 20h
00007FF6FB8158CB  je           static void Fabled_Engine::main+264Bh (00007FF6FB8158EBh)
00007FF6FB8158CD  movzx        ebp, ax
00007FF6FB8158D0  test         dx, dx
00007FF6FB8158D3  je           static void Fabled_Engine::main+263Fh (00007FF6FB8158DFh)
00007FF6FB8158D5  cmp          ebp, 135h
00007FF6FB8158DB  jbe          static void Fabled_Engine::main+2647h (00007FF6FB8158E7h)
00007FF6FB8158DD  jmp          static void Fabled_Engine::main+2670h (00007FF6FB815910h)
00007FF6FB8158DF  cmp          ebp, 96h
00007FF6FB8158E5  jae          static void Fabled_Engine::main+2670h (00007FF6FB815910h)
00007FF6FB8158E7  or           r12d, 20h
00007FF6FB8158EB  test         dil, 2h
00007FF6FB8158EF  je           static void Fabled_Engine::main+2670h (00007FF6FB815910h)
00007FF6FB8158F1  test         dx, dx
00007FF6FB8158F4  je           static void Fabled_Engine::main+2663h (00007FF6FB815903h)
00007FF6FB8158F6  movzx        ebp, ax
00007FF6FB8158F9  cmp          ebp, 135h
00007FF6FB8158FF  jbe          static void Fabled_Engine::main+2669h (00007FF6FB815909h)
00007FF6FB815901  jmp          static void Fabled_Engine::main+2670h (00007FF6FB815910h)
00007FF6FB815903  cmp          ax, 78h
00007FF6FB815907  jae          static void Fabled_Engine::main+2670h (00007FF6FB815910h)
00007FF6FB815909  or           r12d, 2h
00007FF6FB81590D  nop          dword ptr [rax], eax
00007FF6FB815910  test         edi, 100h
00007FF6FB815916  je           static void Fabled_Engine::main+269Dh (00007FF6FB81593Dh)
00007FF6FB815918  movzx        ebp, ax
00007FF6FB81591B  test         dx, dx
00007FF6FB81591E  je           static void Fabled_Engine::main+268Ah (00007FF6FB81592Ah)
00007FF6FB815920  cmp          ebp, 135h
00007FF6FB815926  jbe          static void Fabled_Engine::main+2696h (00007FF6FB815936h)
00007FF6FB815928  jmp          static void Fabled_Engine::main+26C6h (00007FF6FB815966h)
00007FF6FB81592A  cmp          ebp, 82h
00007FF6FB815930  jae          static void Fabled_Engine::main+2719h (00007FF6FB8159B9h)
00007FF6FB815936  or           r12d, 100h
00007FF6FB81593D  test         edi, 200h
00007FF6FB815943  je           static void Fabled_Engine::main+26C6h (00007FF6FB815966h)
00007FF6FB815945  movzx        ebp, ax
00007FF6FB815948  test         dx, dx
00007FF6FB81594B  je           static void Fabled_Engine::main+26B7h (00007FF6FB815957h)
00007FF6FB81594D  cmp          ebp, 12Bh
00007FF6FB815953  jbe          static void Fabled_Engine::main+26BFh (00007FF6FB81595Fh)
00007FF6FB815955  jmp          static void Fabled_Engine::main+26C6h (00007FF6FB815966h)
00007FF6FB815957  cmp          ebp, 82h
00007FF6FB81595D  jae          static void Fabled_Engine::main+2719h (00007FF6FB8159B9h)
00007FF6FB81595F  or           r12d, 200h
00007FF6FB815966  test         dx, dx
00007FF6FB815969  setne        dl
00007FF6FB81596C  je           static void Fabled_Engine::main+271Bh (00007FF6FB8159BBh)
00007FF6FB81596E  mov          ecx, edi
00007FF6FB815970  and          ecx, 400h
00007FF6FB815976  je           static void Fabled_Engine::main+271Bh (00007FF6FB8159BBh)
00007FF6FB815978  or           r12d, 400h
00007FF6FB81597F  mov          dl, 1h
00007FF6FB815981  test         edi, 800h
00007FF6FB815987  jne          static void Fabled_Engine::main+2733h (00007FF6FB8159D3h)
00007FF6FB815989  test         edi, 1000h
00007FF6FB81598F  jne          static void Fabled_Engine::main+276Ch (00007FF6FB815A0Ch)
00007FF6FB815991  test         edi, 2000h
00007FF6FB815997  jne          static void Fabled_Engine::main+279Fh (00007FF6FB815A3Fh)
00007FF6FB81599D  test         edi, 4000h
00007FF6FB8159A3  jne          static void Fabled_Engine::main+27CCh (00007FF6FB815A6Ch)
00007FF6FB8159A9  mov          dl, 1h
00007FF6FB8159AB  test         di, di
00007FF6FB8159AE  jns          static void Fabled_Engine::main+2880h (00007FF6FB815B20h)
00007FF6FB8159B4  jmp          static void Fabled_Engine::main+2836h (00007FF6FB815AD6h)
00007FF6FB8159B9  xor          edx, edx
00007FF6FB8159BB  movzx        ecx, ax
00007FF6FB8159BE  cmp          ecx, 82h
00007FF6FB8159C4  setb         bl
00007FF6FB8159C7  test         edi, 800h
00007FF6FB8159CD  je           static void Fabled_Engine::main+273Ah (00007FF6FB8159DAh)
00007FF6FB8159CF  or           bl, dl
00007FF6FB8159D1  je           static void Fabled_Engine::main+273Ah (00007FF6FB8159DAh)
00007FF6FB8159D3  or           r12d, 800h
00007FF6FB8159DA  test         edi, 1000h
00007FF6FB8159E0  jne          static void Fabled_Engine::main+2760h (00007FF6FB815A00h)
00007FF6FB8159E2  lea          rbx, [rsp+2C0h]
00007FF6FB8159EA  test         edi, 2000h
00007FF6FB8159F0  jne          static void Fabled_Engine::main+2797h (00007FF6FB815A37h)
00007FF6FB8159F2  jmp          static void Fabled_Engine::main+27C0h (00007FF6FB815A60h)
00007FF6FB8159F4  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB8159FE  nop
00007FF6FB815A00  test         dl, dl
00007FF6FB815A02  lea          rbx, [rsp+2C0h]
00007FF6FB815A0A  je           static void Fabled_Engine::main+277Bh (00007FF6FB815A1Bh)
00007FF6FB815A0C  movzx        ecx, ax
00007FF6FB815A0F  mov          dl, 1h
00007FF6FB815A11  cmp          ecx, 13Fh
00007FF6FB815A17  jbe          static void Fabled_Engine::main+2788h (00007FF6FB815A28h)
00007FF6FB815A19  jmp          static void Fabled_Engine::main+27C0h (00007FF6FB815A60h)
00007FF6FB815A1B  movzx        ecx, ax
00007FF6FB815A1E  xor          edx, edx
00007FF6FB815A20  cmp          ecx, 190h
00007FF6FB815A26  jae          static void Fabled_Engine::main+27C0h (00007FF6FB815A60h)
00007FF6FB815A28  or           r12d, 1000h
00007FF6FB815A2F  test         edi, 2000h
00007FF6FB815A35  je           static void Fabled_Engine::main+27C0h (00007FF6FB815A60h)
00007FF6FB815A37  test         dl, dl
00007FF6FB815A39  je           static void Fabled_Engine::main+2842h (00007FF6FB815AE2h)
00007FF6FB815A3F  movzx        ecx, ax
00007FF6FB815A42  mov          dl, 1h
00007FF6FB815A44  cmp          ecx, 12Bh
00007FF6FB815A4A  ja           static void Fabled_Engine::main+27C0h (00007FF6FB815A60h)
00007FF6FB815A4C  or           r12d, 2000h
00007FF6FB815A53  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB815A5D  nop          dword ptr [rax], eax
00007FF6FB815A60  test         edi, 4000h
00007FF6FB815A66  je           static void Fabled_Engine::main+27EFh (00007FF6FB815A8Fh)
00007FF6FB815A68  test         dl, dl
00007FF6FB815A6A  je           static void Fabled_Engine::main+27DBh (00007FF6FB815A7Bh)
00007FF6FB815A6C  movzx        ecx, ax
00007FF6FB815A6F  mov          dl, 1h
00007FF6FB815A71  cmp          ecx, 12Bh
00007FF6FB815A77  jbe          static void Fabled_Engine::main+27E8h (00007FF6FB815A88h)
00007FF6FB815A79  jmp          static void Fabled_Engine::main+27F4h (00007FF6FB815A94h)
00007FF6FB815A7B  movzx        ecx, ax
00007FF6FB815A7E  cmp          ecx, 1C2h
00007FF6FB815A84  jae          static void Fabled_Engine::main+27F4h (00007FF6FB815A94h)
00007FF6FB815A86  xor          edx, edx
00007FF6FB815A88  or           r12d, 4000h
00007FF6FB815A8F  test         di, di
00007FF6FB815A92  js           static void Fabled_Engine::main+2832h (00007FF6FB815AD2h)
00007FF6FB815A94  mov          dl, 1h
00007FF6FB815A96  test         r12d, r12d
00007FF6FB815A99  jne          static void Fabled_Engine::main+2880h (00007FF6FB815B20h)
00007FF6FB815A9F  mov          rax, qword ptr [rsp+2C0h]
00007FF6FB815AA7  mov          qword ptr [rsp+238h], rax
00007FF6FB815AAF  mov          r8d, F0h
00007FF6FB815AB5  lea          rcx, [rsp+970h]
00007FF6FB815ABD  lea          rdx, [rsp+2C8h]
00007FF6FB815AC5  call         memcpy (00007FF6FB91E6A7h)
00007FF6FB815ACA  mov          r12d, esi
00007FF6FB815ACD  jmp          static void Fabled_Engine::main+2B9Bh (00007FF6FB815E3Bh)
00007FF6FB815AD2  test         dl, dl
00007FF6FB815AD4  je           static void Fabled_Engine::main+2858h (00007FF6FB815AF8h)
00007FF6FB815AD6  movzx        eax, ax
00007FF6FB815AD9  cmp          eax, 12Bh
00007FF6FB815ADE  ja           static void Fabled_Engine::main+27F4h (00007FF6FB815A94h)
00007FF6FB815AE0  jmp          static void Fabled_Engine::main+2862h (00007FF6FB815B02h)
00007FF6FB815AE2  movzx        ecx, ax
00007FF6FB815AE5  xor          edx, edx
00007FF6FB815AE7  cmp          ecx, 82h
00007FF6FB815AED  jb           static void Fabled_Engine::main+27ACh (00007FF6FB815A4Ch)
00007FF6FB815AF3  jmp          static void Fabled_Engine::main+27C0h (00007FF6FB815A60h)
00007FF6FB815AF8  movzx        eax, ax
00007FF6FB815AFB  cmp          eax, 190h
00007FF6FB815B00  jae          static void Fabled_Engine::main+27F4h (00007FF6FB815A94h)
00007FF6FB815B02  or           r12d, 8000h
00007FF6FB815B09  mov          dl, 1h
00007FF6FB815B0B  jmp          static void Fabled_Engine::main+2880h (00007FF6FB815B20h)
00007FF6FB815B0D  mov          dl, 2h
00007FF6FB815B0F  mov          r12d, dword ptr [rsp+C8h]
00007FF6FB815B17  nop          word ptr [rax+rax*1], ax
00007FF6FB815B20  mov          al, byte ptr [rsp+102h]
00007FF6FB815B27  lea          rcx, [rsp+239h]
00007FF6FB815B2F  mov          byte ptr [rcx+2h], al
00007FF6FB815B32  movzx        eax, word ptr [rsp+100h]
00007FF6FB815B3A  mov          word ptr [rcx], ax
00007FF6FB815B3D  movdqu       xmm0, xmmword ptr [rsp+40h]
00007FF6FB815B43  movdqa       xmmword ptr [rsp+970h], xmm0
00007FF6FB815B4C  mov          rax, qword ptr [rsp+50h]
00007FF6FB815B51  mov          qword ptr [rsp+980h], rax
00007FF6FB815B59  mov          byte ptr [rsp+238h], dl
00007FF6FB815B60  mov          dword ptr [rsp+23Ch], r12d
00007FF6FB815B68  mov          qword ptr [rsp+230h], 1h
00007FF6FB815B74  mov          r14, qword ptr [rsp+2E0h]
00007FF6FB815B7C  test         r14, r14
00007FF6FB815B7F  je           static void Fabled_Engine::main+29DDh (00007FF6FB815C7Dh)
00007FF6FB815B85  cmp          qword ptr [rsp+2F8h], 0h
00007FF6FB815B8E  je           static void Fabled_Engine::main+29A4h (00007FF6FB815C44h)
00007FF6FB815B94  mov          rbp, qword ptr [rsp+2E8h]
00007FF6FB815B9C  lea          r15, [r14+1h]
00007FF6FB815BA0  lea          rsi, [r14+rbp*1]
00007FF6FB815BA4  add          rsi, 1h
00007FF6FB815BA8  movdqa       xmm0, xmmword ptr [rbp]
00007FF6FB815BAD  pmovmskb     eax, xmm0
00007FF6FB815BB1  not          eax
00007FF6FB815BB3  lea          rdi, [rbp+10h]
00007FF6FB815BB7  nop          word ptr [rax+rax*1], ax
00007FF6FB815BC0  test         ax, ax
00007FF6FB815BC3  je           static void Fabled_Engine::main+2930h (00007FF6FB815BD0h)
00007FF6FB815BC5  lea          ebx, [rax-1h]
00007FF6FB815BC8  and          ebx, eax
00007FF6FB815BCA  jmp          static void Fabled_Engine::main+296Bh (00007FF6FB815C0Bh)
00007FF6FB815BCC  nop          dword ptr [rax], eax
00007FF6FB815BD0  lea          rbx, [rsp+2C0h]
00007FF6FB815BD8  nop          dword ptr [rax+rax*1], eax
00007FF6FB815BE0  cmp          rdi, rsi
00007FF6FB815BE3  jae          static void Fabled_Engine::main+29A8h (00007FF6FB815C48h)
00007FF6FB815BE5  movdqa       xmm0, xmmword ptr [rdi]
00007FF6FB815BE9  pmovmskb     eax, xmm0
00007FF6FB815BED  add          rbp, FFFFFFFFFFFFFD80h
00007FF6FB815BF4  add          rdi, 10h
00007FF6FB815BF8  cmp          ax, FFFFh
00007FF6FB815BFC  je           static void Fabled_Engine::main+2940h (00007FF6FB815BE0h)
00007FF6FB815BFE  mov          ecx, FFFFFFFEh
00007FF6FB815C03  sub          ecx, eax
00007FF6FB815C05  not          eax
00007FF6FB815C07  mov          ebx, eax
00007FF6FB815C09  and          ebx, ecx
00007FF6FB815C0B  bsf          ax, ax
00007FF6FB815C0F  movzx        eax, ax
00007FF6FB815C12  neg          rax
00007FF6FB815C15  lea          rcx, [rax+rax*4]
00007FF6FB815C19  mov          r8, qword ptr [rbp+rcx*8-28h]
00007FF6FB815C1E  mov          eax, ebx
00007FF6FB815C20  test         r8, r8
00007FF6FB815C23  je           static void Fabled_Engine::main+2920h (00007FF6FB815BC0h)
00007FF6FB815C25  cmp          qword ptr [rbp+rcx*8-20h], 0h
00007FF6FB815C2B  mov          eax, ebx
00007FF6FB815C2D  je           static void Fabled_Engine::main+2920h (00007FF6FB815BC0h)
00007FF6FB815C2F  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB815C36  xor          edx, edx
00007FF6FB815C38  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB815C3D  mov          eax, ebx
00007FF6FB815C3F  jmp          static void Fabled_Engine::main+2920h (00007FF6FB815BC0h)
00007FF6FB815C44  lea          r15, [r14+1h]
00007FF6FB815C48  mov          rax, r15
00007FF6FB815C4B  mov          ecx, 28h
00007FF6FB815C50  mul          rcx
00007FF6FB815C53  add          rax, Fh
00007FF6FB815C57  and          rax, FFFFFFFFFFFFFFF0h
00007FF6FB815C5B  add          r14, rax
00007FF6FB815C5E  cmp          r14, FFFFFFFFFFFFFFEFh
00007FF6FB815C62  je           static void Fabled_Engine::main+29DDh (00007FF6FB815C7Dh)
00007FF6FB815C64  mov          r8, qword ptr [rsp+2E8h]
00007FF6FB815C6C  sub          r8, rax
00007FF6FB815C6F  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB815C76  xor          edx, edx
00007FF6FB815C78  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB815C7D  mov          r14, qword ptr [rsp+300h]
00007FF6FB815C85  test         r14, r14
00007FF6FB815C88  je           static void Fabled_Engine::main+2ADDh (00007FF6FB815D7Dh)
00007FF6FB815C8E  cmp          qword ptr [rsp+318h], 0h
00007FF6FB815C97  je           static void Fabled_Engine::main+2AA4h (00007FF6FB815D44h)
00007FF6FB815C9D  mov          rbp, qword ptr [rsp+308h]
00007FF6FB815CA5  lea          r15, [r14+1h]
00007FF6FB815CA9  lea          rsi, [r14+rbp*1]
00007FF6FB815CAD  add          rsi, 1h
00007FF6FB815CB1  movdqa       xmm0, xmmword ptr [rbp]
00007FF6FB815CB6  pmovmskb     eax, xmm0
00007FF6FB815CBA  not          eax
00007FF6FB815CBC  lea          rdi, [rbp+10h]
00007FF6FB815CC0  test         ax, ax
00007FF6FB815CC3  je           static void Fabled_Engine::main+2A30h (00007FF6FB815CD0h)
00007FF6FB815CC5  lea          ebx, [rax-1h]
00007FF6FB815CC8  and          ebx, eax
00007FF6FB815CCA  jmp          static void Fabled_Engine::main+2A6Bh (00007FF6FB815D0Bh)
00007FF6FB815CCC  nop          dword ptr [rax], eax
00007FF6FB815CD0  lea          rbx, [rsp+2C0h]
00007FF6FB815CD8  nop          dword ptr [rax+rax*1], eax
00007FF6FB815CE0  cmp          rdi, rsi
00007FF6FB815CE3  jae          static void Fabled_Engine::main+2AA8h (00007FF6FB815D48h)
00007FF6FB815CE5  movdqa       xmm0, xmmword ptr [rdi]
00007FF6FB815CE9  pmovmskb     eax, xmm0
00007FF6FB815CED  add          rbp, FFFFFFFFFFFFFE80h
00007FF6FB815CF4  add          rdi, 10h
00007FF6FB815CF8  cmp          ax, FFFFh
00007FF6FB815CFC  je           static void Fabled_Engine::main+2A40h (00007FF6FB815CE0h)
00007FF6FB815CFE  mov          ecx, FFFFFFFEh
00007FF6FB815D03  sub          ecx, eax
00007FF6FB815D05  not          eax
00007FF6FB815D07  mov          ebx, eax
00007FF6FB815D09  and          ebx, ecx
00007FF6FB815D0B  bsf          ax, ax
00007FF6FB815D0F  movzx        eax, ax
00007FF6FB815D12  neg          rax
00007FF6FB815D15  lea          rcx, [rax+rax*2]
00007FF6FB815D19  mov          r8, qword ptr [rbp+rcx*8-18h]
00007FF6FB815D1E  mov          eax, ebx
00007FF6FB815D20  test         r8, r8
00007FF6FB815D23  je           static void Fabled_Engine::main+2A20h (00007FF6FB815CC0h)
00007FF6FB815D25  cmp          qword ptr [rbp+rcx*8-10h], 0h
00007FF6FB815D2B  mov          eax, ebx
00007FF6FB815D2D  je           static void Fabled_Engine::main+2A20h (00007FF6FB815CC0h)
00007FF6FB815D2F  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB815D36  xor          edx, edx
00007FF6FB815D38  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB815D3D  mov          eax, ebx
00007FF6FB815D3F  jmp          static void Fabled_Engine::main+2A20h (00007FF6FB815CC0h)
00007FF6FB815D44  lea          r15, [r14+1h]
00007FF6FB815D48  mov          rax, r15
00007FF6FB815D4B  mov          ecx, 18h
00007FF6FB815D50  mul          rcx
00007FF6FB815D53  add          rax, Fh
00007FF6FB815D57  and          rax, FFFFFFFFFFFFFFF0h
00007FF6FB815D5B  add          r14, rax
00007FF6FB815D5E  cmp          r14, FFFFFFFFFFFFFFEFh
00007FF6FB815D62  je           static void Fabled_Engine::main+2ADDh (00007FF6FB815D7Dh)
00007FF6FB815D64  mov          r8, qword ptr [rsp+308h]
00007FF6FB815D6C  sub          r8, rax
00007FF6FB815D6F  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB815D76  xor          edx, edx
00007FF6FB815D78  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB815D7D  mov          rax, qword ptr [rsp+330h]
00007FF6FB815D85  test         rax, rax
00007FF6FB815D88  je           static void Fabled_Engine::main+2B30h (00007FF6FB815DD0h)
00007FF6FB815D8A  mov          rsi, qword ptr [rsp+320h]
00007FF6FB815D92  lea          rax, [rax+rax*2]
00007FF6FB815D96  lea          rdi, [rsi+rax*8]
00007FF6FB815D9A  jmp          static void Fabled_Engine::main+2B09h (00007FF6FB815DA9h)
00007FF6FB815D9C  nop          dword ptr [rax], eax
00007FF6FB815DA0  add          rsi, 18h
00007FF6FB815DA4  cmp          rsi, rdi
00007FF6FB815DA7  je           static void Fabled_Engine::main+2B30h (00007FF6FB815DD0h)
00007FF6FB815DA9  mov          r8, qword ptr [rsi]
00007FF6FB815DAC  test         r8, r8
00007FF6FB815DAF  je           static void Fabled_Engine::main+2B00h (00007FF6FB815DA0h)
00007FF6FB815DB1  cmp          qword ptr [rsi+8h], 0h
00007FF6FB815DB6  je           static void Fabled_Engine::main+2B00h (00007FF6FB815DA0h)
00007FF6FB815DB8  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB815DBF  xor          edx, edx
00007FF6FB815DC1  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB815DC6  jmp          static void Fabled_Engine::main+2B00h (00007FF6FB815DA0h)
00007FF6FB815DC8  nop          dword ptr [rax+rax*1], eax
00007FF6FB815DD0  mov          rax, qword ptr [rsp+328h]
00007FF6FB815DD8  test         rax, rax
00007FF6FB815DDB  je           static void Fabled_Engine::main+2B65h (00007FF6FB815E05h)
00007FF6FB815DDD  shl          rax, 3h
00007FF6FB815DE1  lea          rax, [rax+rax*2]
00007FF6FB815DE5  test         rax, rax
00007FF6FB815DE8  je           static void Fabled_Engine::main+2B65h (00007FF6FB815E05h)
00007FF6FB815DEA  mov          r8, qword ptr [rsp+320h]
00007FF6FB815DF2  test         r8, r8
00007FF6FB815DF5  je           static void Fabled_Engine::main+2B65h (00007FF6FB815E05h)
00007FF6FB815DF7  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB815DFE  xor          edx, edx
00007FF6FB815E00  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB815E05  lea          rcx, [rsp+340h]
00007FF6FB815E0D  call         static void hashbrown::raw::{{impl}}::drop<tuple<enum$<naga::proc::namer::NameKey>, alloc::string::String>,alloc::alloc::Global> (00007FF6FB8272A0h)
00007FF6FB815E12  lea          rcx, [rsp+360h]
00007FF6FB815E1A  call         static void hashbrown::raw::{{impl}}::drop<tuple<naga::arena::Handle<enum$<naga::Expression>>, alloc::string::String>,alloc::alloc::Global> (00007FF6FB827190h)
00007FF6FB815E1F  lea          rcx, [rsp+388h]
00007FF6FB815E27  call         static void hashbrown::raw::{{impl}}::drop<tuple<naga::arena::Handle<enum$<naga::Expression>>, alloc::string::String>,alloc::alloc::Global> (00007FF6FB827190h)
00007FF6FB815E2C  cmp          qword ptr [rsp+230h], 1h
00007FF6FB815E35  je           static void Fabled_Engine::main+5BEFh (00007FF6FB818E8Fh)
00007FF6FB815E3B  mov          dword ptr [rsp+C8h], r12d
00007FF6FB815E43  mov          rax, qword ptr [rsp+238h]
00007FF6FB815E4B  mov          qword ptr [rsp+810h], rax
00007FF6FB815E53  mov          r8d, F0h
00007FF6FB815E59  lea          rcx, [rsp+818h]
00007FF6FB815E61  lea          rdx, [rsp+970h]
00007FF6FB815E69  call         memcpy (00007FF6FB91E6A7h)
00007FF6FB815E6E  mov          r8d, F8h
00007FF6FB815E74  lea          rsi, [rsp+6C8h]
00007FF6FB815E7C  mov          rcx, rsi
00007FF6FB815E7F  lea          rdx, [rsp+810h]
00007FF6FB815E87  call         memcpy (00007FF6FB91E6A7h)
00007FF6FB815E8C  mov          rcx, rbx
00007FF6FB815E8F  mov          rdx, rsi
00007FF6FB815E92  call         static union enum$<core::result::Result<naga::back::glsl::ReflectionInfo, enum$<naga::back::glsl::Error>>> naga::back::glsl::Writer<mut alloc::string::String*>::write<mut alloc::string::String*> (00007FF6FB81A100h)
00007FF6FB815E97  cmp          dword ptr [rsp+2C0h], 1h
00007FF6FB815E9F  mov          rbp, qword ptr [rsp+2C8h]
00007FF6FB815EA7  mov          r12, qword ptr [rsp+2D0h]
00007FF6FB815EAF  je           static void Fabled_Engine::main+602Bh (00007FF6FB8192CBh)
00007FF6FB815EB5  mov          r14, qword ptr [rsp+2E8h]
00007FF6FB815EBD  mov          r13, qword ptr [rsp+2F0h]
00007FF6FB815EC5  mov          rsi, qword ptr [rsp+300h]
00007FF6FB815ECD  test         rbp, rbp
00007FF6FB815ED0  je           static void Fabled_Engine::main+2D25h (00007FF6FB815FC5h)
00007FF6FB815ED6  cmp          qword ptr [rsp+2E0h], 0h
00007FF6FB815EDF  je           static void Fabled_Engine::main+2CFCh (00007FF6FB815F9Ch)
00007FF6FB815EE5  mov          qword ptr [rsp+28h], rsi
00007FF6FB815EEA  lea          rax, [rbp+1h]
00007FF6FB815EEE  mov          qword ptr [rsp+38h], rax
00007FF6FB815EF3  lea          rsi, [r12+rbp*1]
00007FF6FB815EF7  add          rsi, 1h
00007FF6FB815EFB  movdqa       xmm0, xmmword ptr [r12]
00007FF6FB815F01  pmovmskb     eax, xmm0
00007FF6FB815F05  not          eax
00007FF6FB815F07  lea          r15, [r12+10h]
00007FF6FB815F0C  mov          rbx, r12
00007FF6FB815F0F  nop
00007FF6FB815F10  test         ax, ax
00007FF6FB815F13  je           static void Fabled_Engine::main+2C80h (00007FF6FB815F20h)
00007FF6FB815F15  lea          edi, [rax-1h]
00007FF6FB815F18  and          edi, eax
00007FF6FB815F1A  jmp          static void Fabled_Engine::main+2CACh (00007FF6FB815F4Ch)
00007FF6FB815F1C  nop          dword ptr [rax], eax
00007FF6FB815F20  cmp          r15, rsi
00007FF6FB815F23  jae          static void Fabled_Engine::main+2CF0h (00007FF6FB815F90h)
00007FF6FB815F25  movdqa       xmm0, xmmword ptr [r15]
00007FF6FB815F2A  pmovmskb     eax, xmm0
00007FF6FB815F2E  add          rbx, FFFFFFFFFFFFFE00h
00007FF6FB815F35  add          r15, 10h
00007FF6FB815F39  cmp          ax, FFFFh
00007FF6FB815F3D  je           static void Fabled_Engine::main+2C80h (00007FF6FB815F20h)
00007FF6FB815F3F  mov          ecx, FFFFFFFEh
00007FF6FB815F44  sub          ecx, eax
00007FF6FB815F46  not          eax
00007FF6FB815F48  mov          edi, eax
00007FF6FB815F4A  and          edi, ecx
00007FF6FB815F4C  bsf          ax, ax
00007FF6FB815F50  movzx        eax, ax
00007FF6FB815F53  shl          rax, 5h
00007FF6FB815F57  mov          rcx, rbx
00007FF6FB815F5A  sub          rcx, rax
00007FF6FB815F5D  mov          r8, qword ptr [rcx-20h]
00007FF6FB815F61  mov          eax, edi
00007FF6FB815F63  test         r8, r8
00007FF6FB815F66  je           static void Fabled_Engine::main+2C70h (00007FF6FB815F10h)
00007FF6FB815F68  cmp          qword ptr [rcx-18h], 0h
00007FF6FB815F6D  mov          eax, edi
00007FF6FB815F6F  je           static void Fabled_Engine::main+2C70h (00007FF6FB815F10h)
00007FF6FB815F71  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB815F78  xor          edx, edx
00007FF6FB815F7A  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB815F7F  mov          eax, edi
00007FF6FB815F81  jmp          static void Fabled_Engine::main+2C70h (00007FF6FB815F10h)
00007FF6FB815F83  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB815F8D  nop          dword ptr [rax], eax
00007FF6FB815F90  mov          rsi, qword ptr [rsp+28h]
00007FF6FB815F95  mov          rax, qword ptr [rsp+38h]
00007FF6FB815F9A  jmp          static void Fabled_Engine::main+2D00h (00007FF6FB815FA0h)
00007FF6FB815F9C  lea          rax, [rbp+1h]
00007FF6FB815FA0  mov          ecx, 20h
00007FF6FB815FA5  mul          rcx
00007FF6FB815FA8  add          rbp, rax
00007FF6FB815FAB  cmp          rbp, FFFFFFFFFFFFFFEFh
00007FF6FB815FAF  je           static void Fabled_Engine::main+2D25h (00007FF6FB815FC5h)
00007FF6FB815FB1  sub          r12, rax
00007FF6FB815FB4  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB815FBB  xor          edx, edx
00007FF6FB815FBD  mov          r8, r12
00007FF6FB815FC0  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB815FC5  test         r14, r14
00007FF6FB815FC8  mov          r12, qword ptr [rsp+30h]
00007FF6FB815FCD  je           static void Fabled_Engine::main+2E08h (00007FF6FB8160A8h)
00007FF6FB815FD3  lea          r15, [r14+1h]
00007FF6FB815FD7  test         rsi, rsi
00007FF6FB815FDA  je           static void Fabled_Engine::main+2DE0h (00007FF6FB816080h)
00007FF6FB815FE0  lea          rsi, [r14+r13*1]
00007FF6FB815FE4  add          rsi, 1h
00007FF6FB815FE8  movdqa       xmm0, xmmword ptr [r13]
00007FF6FB815FEE  pmovmskb     eax, xmm0
00007FF6FB815FF2  not          eax
00007FF6FB815FF4  lea          rbp, [r13+10h]
00007FF6FB815FF8  mov          rbx, r13
00007FF6FB815FFB  nop          dword ptr [rax+rax*1], eax
00007FF6FB816000  test         ax, ax
00007FF6FB816003  je           static void Fabled_Engine::main+2D70h (00007FF6FB816010h)
00007FF6FB816005  lea          edi, [rax-1h]
00007FF6FB816008  and          edi, eax
00007FF6FB81600A  jmp          static void Fabled_Engine::main+2D9Ch (00007FF6FB81603Ch)
00007FF6FB81600C  nop          dword ptr [rax], eax
00007FF6FB816010  cmp          rbp, rsi
00007FF6FB816013  jae          static void Fabled_Engine::main+2DE0h (00007FF6FB816080h)
00007FF6FB816015  movdqa       xmm0, xmmword ptr [rbp]
00007FF6FB81601A  pmovmskb     eax, xmm0
00007FF6FB81601E  add          rbx, FFFFFFFFFFFFFE00h
00007FF6FB816025  add          rbp, 10h
00007FF6FB816029  cmp          ax, FFFFh
00007FF6FB81602D  je           static void Fabled_Engine::main+2D70h (00007FF6FB816010h)
00007FF6FB81602F  mov          ecx, FFFFFFFEh
00007FF6FB816034  sub          ecx, eax
00007FF6FB816036  not          eax
00007FF6FB816038  mov          edi, eax
00007FF6FB81603A  and          edi, ecx
00007FF6FB81603C  bsf          ax, ax
00007FF6FB816040  movzx        eax, ax
00007FF6FB816043  shl          rax, 5h
00007FF6FB816047  mov          rcx, rbx
00007FF6FB81604A  sub          rcx, rax
00007FF6FB81604D  mov          r8, qword ptr [rcx-18h]
00007FF6FB816051  mov          eax, edi
00007FF6FB816053  test         r8, r8
00007FF6FB816056  je           static void Fabled_Engine::main+2D60h (00007FF6FB816000h)
00007FF6FB816058  cmp          qword ptr [rcx-10h], 0h
00007FF6FB81605D  mov          eax, edi
00007FF6FB81605F  je           static void Fabled_Engine::main+2D60h (00007FF6FB816000h)
00007FF6FB816061  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB816068  xor          edx, edx
00007FF6FB81606A  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB81606F  mov          eax, edi
00007FF6FB816071  jmp          static void Fabled_Engine::main+2D60h (00007FF6FB816000h)
00007FF6FB816073  nop          word ptr cs:[rax+rax*1], ax
00007FF6FB81607D  nop          dword ptr [rax], eax
00007FF6FB816080  mov          rax, r15
00007FF6FB816083  mov          ecx, 20h
00007FF6FB816088  mul          rcx
00007FF6FB81608B  add          r14, rax
00007FF6FB81608E  cmp          r14, FFFFFFFFFFFFFFEFh
00007FF6FB816092  je           static void Fabled_Engine::main+2E08h (00007FF6FB8160A8h)
00007FF6FB816094  sub          r13, rax
00007FF6FB816097  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB81609E  xor          edx, edx
00007FF6FB8160A0  mov          r8, r13
00007FF6FB8160A3  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB8160A8  lea          rcx, [rsp+6C8h]
00007FF6FB8160B0  call         static void core::ptr::drop_in_place<naga::back::glsl::Writer<mut alloc::string::String*>> (00007FF6FB8122B0h)
00007FF6FB8160B5  mov          r8, qword ptr [rsp+270h]
00007FF6FB8160BD  test         r8, r8
00007FF6FB8160C0  je           static void Fabled_Engine::main+2E3Bh (00007FF6FB8160DBh)
00007FF6FB8160C2  cmp          qword ptr [rsp+278h], 0h
00007FF6FB8160CB  je           static void Fabled_Engine::main+2E3Bh (00007FF6FB8160DBh)
00007FF6FB8160CD  mov          rcx, qword ptr [00007FF6FB94E1D8h]
00007FF6FB8160D4  xor          edx, edx
00007FF6FB8160D6  call         HeapFree (00007FF6FB91D5BCh)
00007FF6FB8160DB  cmp          r12, qword ptr [rsp+C0h]
00007FF6FB8160E3  jne          static void Fabled_Engine::main+682h (00007FF6FB813922h)
00007FF6FB8160E9  mov          rsi, qword ptr [rsp+5F0h]
00007FF6FB8160F1  cmp          qword ptr [rsp+5F8h], 0h
00007FF6FB8160FA  sete         bl
00007FF6FB8160FD  jmp          static void Fabled_Engine::main+2E61h (00007FF6FB816101h)
00007FF6FB8160FF  mov          bl, 1h
00007FF6FB816101  lea          rcx, [rsp+150h]
00007FF6FB816109  call         static void core::ptr::drop_in_place<alloc::vec::Vec<naga::valid::analyzer::FunctionInfo, alloc::alloc::Global>> (00007FF6FB812060h)
00007FF6FB81610E  lea          rcx, [rsp+168h]
00007FF6FB816116  call         static void core::ptr::drop_in_place<alloc::vec::Vec<naga::valid::analyzer::FunctionInfo, alloc::alloc::Global>> (00007FF6FB812060h)
00007FF6FB81611B  lea          rcx, [rsp+A68h]
00007FF6FB816123  call         static void core::ptr::drop_in_place<naga::Module> (00007FF6FB811150h)
00007FF6FB816128  lea          rcx, [rsp+2C0h]
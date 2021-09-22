# --------------- Cube Dissassembly -------------------
//    let plane = fabled_render::mesh::Plane::default();
00007FF7EDD7133E  mov          rcx, qword ptr [00007FF7EDD931C8h]
00007FF7EDD71345  test         rcx, rcx
00007FF7EDD71348  jne          static void Fabled_Engine::main+A2h (00007FF7EDD71362h)
00007FF7EDD7134A  call         GetProcessHeap (00007FF7EDD88EA2h)
00007FF7EDD7134F  test         rax, rax
00007FF7EDD71352  je           static void Fabled_Engine::main+6DAh (00007FF7EDD7199Ah)
00007FF7EDD71358  mov          rcx, rax
00007FF7EDD7135B  mov          qword ptr [00007FF7EDD931C8h], rax
00007FF7EDD71362  mov          r8d, 1E40h
00007FF7EDD71368  xor          edx, edx
00007FF7EDD7136A  call         HeapAlloc (00007FF7EDD88EA8h)
00007FF7EDD7136F  test         rax, rax
00007FF7EDD71372  je           static void Fabled_Engine::main+6DAh (00007FF7EDD7199Ah)
00007FF7EDD71378  mov          qword ptr [rsp+48h], rax
00007FF7EDD7137D  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000079 (00007FF7EDD8C520h)]
00007FF7EDD71384  movups       xmmword ptr [rsp+50h], xmm0
00007FF7EDD71389  mov          rcx, qword ptr [00007FF7EDD931C8h]
00007FF7EDD71390  test         rcx, rcx
00007FF7EDD71393  jne          static void Fabled_Engine::main+EDh (00007FF7EDD713ADh)
00007FF7EDD71395  call         GetProcessHeap (00007FF7EDD88EA2h)
00007FF7EDD7139A  test         rax, rax
00007FF7EDD7139D  je           static void Fabled_Engine::main+6E1h (00007FF7EDD719A1h)
00007FF7EDD713A3  mov          rcx, rax
00007FF7EDD713A6  mov          qword ptr [00007FF7EDD931C8h], rax
00007FF7EDD713AD  mov          r8d, 12C0h
00007FF7EDD713B3  xor          edx, edx
00007FF7EDD713B5  call         HeapAlloc (00007FF7EDD88EA8h)
00007FF7EDD713BA  test         rax, rax
00007FF7EDD713BD  je           static void Fabled_Engine::main+6E1h (00007FF7EDD719A1h)
00007FF7EDD713C3  mov          qword ptr [rsp+28h], rax
00007FF7EDD713C8  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000258 (00007FF7EDD8C530h)]
00007FF7EDD713CF  movups       xmmword ptr [rsp+30h], xmm0
00007FF7EDD713D4  xor          esi, esi
00007FF7EDD713D6  movss        xmm8, dword ptr [__real@3dcccccd (00007FF7EDD8C540h)]
00007FF7EDD713DF  movss        xmm9, dword ptr [__real@bf000000 (00007FF7EDD8C544h)]
00007FF7EDD713E8  xorps        xmm10, xmm10
00007FF7EDD713EC  lea          r14, [rsp+48h]
00007FF7EDD713F1  mov          edi, dword ptr [00007FF7EDD8DA58h]
00007FF7EDD713F7  mov          rbx, qword ptr [00007FF7EDD8DA50h]
00007FF7EDD713FE  jmp          static void Fabled_Engine::main+14Dh (00007FF7EDD7140Dh)
00007FF7EDD71400  cmp          rsi, Ah
00007FF7EDD71404  mov          rsi, r15
00007FF7EDD71407  je           static void Fabled_Engine::main+26Dh (00007FF7EDD7152Dh)
00007FF7EDD7140D  test         rsi, rsi
00007FF7EDD71410  js           static void Fabled_Engine::main+160h (00007FF7EDD71420h)
00007FF7EDD71412  xorps        xmm11, xmm11
00007FF7EDD71416  cvtsi2ss     xmm11, rsi
00007FF7EDD7141B  jmp          static void Fabled_Engine::main+17Ch (00007FF7EDD7143Ch)
00007FF7EDD7141D  nop          dword ptr [rax], eax
00007FF7EDD71420  mov          rax, rsi
00007FF7EDD71423  shr          rax, 1h
00007FF7EDD71426  mov          ecx, esi
00007FF7EDD71428  and          ecx, 1h
00007FF7EDD7142B  or           rcx, rax
00007FF7EDD7142E  xorps        xmm11, xmm11
00007FF7EDD71432  cvtsi2ss     xmm11, rcx
00007FF7EDD71437  addss        xmm11, xmm11
00007FF7EDD7143C  lea          r15, [rsi+1h]
00007FF7EDD71440  mulss        xmm11, xmm8
00007FF7EDD71445  movaps       xmm12, xmm11
00007FF7EDD71449  addss        xmm12, xmm9
00007FF7EDD7144E  mov          rdx, qword ptr [rsp+58h]
00007FF7EDD71453  xor          ebp, ebp
00007FF7EDD71455  jmp          static void Fabled_Engine::main+233h (00007FF7EDD714F3h)
00007FF7EDD7145A  nop          word ptr [rax+rax*1], ax
00007FF7EDD71460  xorps        xmm6, xmm6
00007FF7EDD71463  cvtsi2ss     xmm6, rbp
00007FF7EDD71468  mulss        xmm6, xmm8
00007FF7EDD7146D  movaps       xmm7, xmm6
00007FF7EDD71470  addss        xmm7, xmm9
00007FF7EDD71475  movaps       xmmword ptr [rsp+60h], xmm10
00007FF7EDD7147B  add          rbp, 1h
00007FF7EDD7147F  movaps       xmmword ptr [rsp+90h], xmm10
00007FF7EDD71488  cmp          rdx, qword ptr [rsp+50h]
00007FF7EDD7148D  je           static void Fabled_Engine::main+25Bh (00007FF7EDD7151Bh)
00007FF7EDD71493  mov          rax, qword ptr [rsp+48h]
00007FF7EDD71498  mov          rcx, rdx
00007FF7EDD7149B  shl          rcx, 6h
00007FF7EDD7149F  movss        dword ptr [rax+rcx*1], xmm7
00007FF7EDD714A4  mov          dword ptr [rax+rcx*1+4h], 0h
00007FF7EDD714AC  movss        dword ptr [rax+rcx*1+8h], xmm12
00007FF7EDD714B3  movss        dword ptr [rax+rcx*1+Ch], xmm6
00007FF7EDD714B9  movss        dword ptr [rax+rcx*1+10h], xmm11
00007FF7EDD714C0  mov          dword ptr [rax+rcx*1+1Ch], edi
00007FF7EDD714C4  mov          qword ptr [rax+rcx*1+14h], rbx
00007FF7EDD714C9  movaps       xmm0, xmmword ptr [rsp+60h]
00007FF7EDD714CE  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF7EDD714D3  movaps       xmm0, xmmword ptr [rsp+90h]
00007FF7EDD714DB  movaps       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF7EDD714E0  add          rdx, 1h
00007FF7EDD714E4  mov          qword ptr [rsp+58h], rdx
00007FF7EDD714E9  cmp          rbp, Bh
00007FF7EDD714ED  je           static void Fabled_Engine::main+140h (00007FF7EDD71400h)
00007FF7EDD714F3  test         rbp, rbp
00007FF7EDD714F6  jns          static void Fabled_Engine::main+1A0h (00007FF7EDD71460h)
00007FF7EDD714FC  mov          rax, rbp
00007FF7EDD714FF  shr          rax, 1h
00007FF7EDD71502  mov          ecx, ebp
00007FF7EDD71504  and          ecx, 1h
00007FF7EDD71507  or           rcx, rax
00007FF7EDD7150A  xorps        xmm6, xmm6
00007FF7EDD7150D  cvtsi2ss     xmm6, rcx
00007FF7EDD71512  addss        xmm6, xmm6
00007FF7EDD71516  jmp          static void Fabled_Engine::main+1A8h (00007FF7EDD71468h)
00007FF7EDD7151B  mov          rcx, r14
00007FF7EDD7151E  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF7EDD8A9D0h)
00007FF7EDD71523  mov          rdx, qword ptr [rsp+58h]
00007FF7EDD71528  jmp          static void Fabled_Engine::main+1D3h (00007FF7EDD71493h)
00007FF7EDD7152D  mov          r13d, Ch
00007FF7EDD71533  xor          r15d, r15d
00007FF7EDD71536  lea          r12, [rsp+28h]
00007FF7EDD7153B  jmp          static void Fabled_Engine::main+28Eh (00007FF7EDD7154Eh)
00007FF7EDD7153D  nop          dword ptr [rax], eax
00007FF7EDD71540  add          r13, Bh
00007FF7EDD71544  cmp          r15, Ah
00007FF7EDD71548  je           static void Fabled_Engine::main+3FAh (00007FF7EDD716BAh)
00007FF7EDD7154E  add          r15, 1h
00007FF7EDD71552  mov          esi, Ah
00007FF7EDD71557  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD7155C  mov          rbx, r13
00007FF7EDD7155F  jmp          static void Fabled_Engine::main+2D1h (00007FF7EDD71591h)
00007FF7EDD71561  nop          word ptr cs:[rax+rax*1], ax
00007FF7EDD7156B  nop          dword ptr [rax+rax*1], eax
00007FF7EDD71570  mov          rax, qword ptr [rsp+28h]
00007FF7EDD71575  mov          qword ptr [rax+rdx*8], rdi
00007FF7EDD71579  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD7157E  add          rdx, 1h
00007FF7EDD71582  mov          qword ptr [rsp+38h], rdx
00007FF7EDD71587  add          rbx, 1h
00007FF7EDD7158B  add          rsi, FFFFFFFFFFFFFFFFh
00007FF7EDD7158F  je           static void Fabled_Engine::main+280h (00007FF7EDD71540h)
00007FF7EDD71591  lea          rdi, [rbx-Ch]
00007FF7EDD71595  cmp          rdx, qword ptr [rsp+30h]
00007FF7EDD7159A  je           static void Fabled_Engine::main+3A0h (00007FF7EDD71660h)
00007FF7EDD715A0  lea          rbp, [rbx-1h]
00007FF7EDD715A4  mov          rax, qword ptr [rsp+28h]
00007FF7EDD715A9  mov          qword ptr [rax+rdx*8], rdi
00007FF7EDD715AD  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD715B2  add          rdx, 1h
00007FF7EDD715B6  mov          qword ptr [rsp+38h], rdx
00007FF7EDD715BB  cmp          rdx, qword ptr [rsp+30h]
00007FF7EDD715C0  je           static void Fabled_Engine::main+3B2h (00007FF7EDD71672h)
00007FF7EDD715C6  mov          rax, qword ptr [rsp+28h]
00007FF7EDD715CB  mov          qword ptr [rax+rdx*8], rbp
00007FF7EDD715CF  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD715D4  add          rdx, 1h
00007FF7EDD715D8  mov          qword ptr [rsp+38h], rdx
00007FF7EDD715DD  lea          rdi, [rbx-Bh]
00007FF7EDD715E1  cmp          rdx, qword ptr [rsp+30h]
00007FF7EDD715E6  je           static void Fabled_Engine::main+3C4h (00007FF7EDD71684h)
00007FF7EDD715EC  mov          rax, qword ptr [rsp+28h]
00007FF7EDD715F1  mov          qword ptr [rax+rdx*8], rdi
00007FF7EDD715F5  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD715FA  add          rdx, 1h
00007FF7EDD715FE  mov          qword ptr [rsp+38h], rdx
00007FF7EDD71603  cmp          rdx, qword ptr [rsp+30h]
00007FF7EDD71608  je           static void Fabled_Engine::main+3D6h (00007FF7EDD71696h)
00007FF7EDD7160E  mov          rax, qword ptr [rsp+28h]
00007FF7EDD71613  mov          qword ptr [rax+rdx*8], rbp
00007FF7EDD71617  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD7161C  add          rdx, 1h
00007FF7EDD71620  mov          qword ptr [rsp+38h], rdx
00007FF7EDD71625  cmp          rdx, qword ptr [rsp+30h]
00007FF7EDD7162A  je           static void Fabled_Engine::main+3E8h (00007FF7EDD716A8h)
00007FF7EDD7162C  mov          rax, qword ptr [rsp+28h]
00007FF7EDD71631  mov          qword ptr [rax+rdx*8], rbx
00007FF7EDD71635  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD7163A  add          rdx, 1h
00007FF7EDD7163E  mov          qword ptr [rsp+38h], rdx
00007FF7EDD71643  cmp          rdx, qword ptr [rsp+30h]
00007FF7EDD71648  jne          static void Fabled_Engine::main+2B0h (00007FF7EDD71570h)
00007FF7EDD7164E  mov          rcx, r12
00007FF7EDD71651  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7EDD8AA90h)
00007FF7EDD71656  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD7165B  jmp          static void Fabled_Engine::main+2B0h (00007FF7EDD71570h)
00007FF7EDD71660  mov          rcx, r12
00007FF7EDD71663  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7EDD8AA90h)
00007FF7EDD71668  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD7166D  jmp          static void Fabled_Engine::main+2E0h (00007FF7EDD715A0h)
00007FF7EDD71672  mov          rcx, r12
00007FF7EDD71675  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7EDD8AA90h)
00007FF7EDD7167A  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD7167F  jmp          static void Fabled_Engine::main+306h (00007FF7EDD715C6h)
00007FF7EDD71684  mov          rcx, r12
00007FF7EDD71687  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7EDD8AA90h)
00007FF7EDD7168C  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD71691  jmp          static void Fabled_Engine::main+32Ch (00007FF7EDD715ECh)
00007FF7EDD71696  mov          rcx, r12
00007FF7EDD71699  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7EDD8AA90h)
00007FF7EDD7169E  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD716A3  jmp          static void Fabled_Engine::main+34Eh (00007FF7EDD7160Eh)
00007FF7EDD716A8  mov          rcx, r12
00007FF7EDD716AB  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7EDD8AA90h)
00007FF7EDD716B0  mov          rdx, qword ptr [rsp+38h]
00007FF7EDD716B5  jmp          static void Fabled_Engine::main+36Ch (00007FF7EDD7162Ch)
00007FF7EDD716BA  mov          rax, qword ptr [rsp+38h]
00007FF7EDD716BF  mov          qword ptr [rsp+74h], rax
00007FF7EDD716C4  movups       xmm0, xmmword ptr [rsp+28h]
00007FF7EDD716C9  movups       xmmword ptr [rsp+64h], xmm0
00007FF7EDD716CE  mov          rcx, qword ptr [00007FF7EDD931C8h]
00007FF7EDD716D5  test         rcx, rcx
00007FF7EDD716D8  jne          static void Fabled_Engine::main+432h (00007FF7EDD716F2h)
00007FF7EDD716DA  call         GetProcessHeap (00007FF7EDD88EA2h)
00007FF7EDD716DF  test         rax, rax
00007FF7EDD716E2  je           static void Fabled_Engine::main+6F2h (00007FF7EDD719B2h)
00007FF7EDD716E8  mov          rcx, rax
00007FF7EDD716EB  mov          qword ptr [00007FF7EDD931C8h], rax
00007FF7EDD716F2  mov          r8d, 40h
00007FF7EDD716F8  xor          edx, edx
00007FF7EDD716FA  call         HeapAlloc (00007FF7EDD88EA8h)
00007FF7EDD716FF  test         rax, rax
00007FF7EDD71702  je           static void Fabled_Engine::main+6F2h (00007FF7EDD719B2h)
00007FF7EDD71708  mov          rbx, rax
00007FF7EDD7170B  mov          rax, qword ptr [rsp+58h]
00007FF7EDD71710  mov          qword ptr [rbx+10h], rax
00007FF7EDD71714  movups       xmm0, xmmword ptr [rsp+48h]
00007FF7EDD71719  movups       xmmword ptr [rbx], xmm0
00007FF7EDD7171C  movups       xmm0, xmmword ptr [rsp+60h]
00007FF7EDD71721  movups       xmm1, xmmword ptr [rsp+6Ch]
00007FF7EDD71726  movups       xmmword ptr [rbx+1Ch], xmm0
00007FF7EDD7172A  movups       xmmword ptr [rbx+28h], xmm1
00007FF7EDD7172E  mov          dword ptr [rbx+18h], 0h
00007FF7EDD71735  lea          rax, [00007FF7EDD8FAA0h]
00007FF7EDD7173C  mov          qword ptr [rsp+90h], rax
00007FF7EDD71744  mov          qword ptr [rsp+98h], 6h
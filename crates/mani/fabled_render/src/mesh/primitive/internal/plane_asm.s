# --------------- Cube Dissassembly -------------------
//    let plane = fabled_render::mesh::Plane::default();
00007FF6FE0612A9  movss        xmm0, dword ptr [__real@41200000 (00007FF6FE07C510h)]
00007FF6FE0612B1  minss        xmm0, xmm0
00007FF6FE0612B5  xorps        xmm1, xmm1
00007FF6FE0612B8  maxss        xmm1, xmm0
00007FF6FE0612BC  movss        xmm0, dword ptr [__real@437f0000 (00007FF6FE07C514h)]
00007FF6FE0612C4  minss        xmm0, xmm1
00007FF6FE0612C8  cvttss2si    esi, xmm0

//    let plane_model: fabled_render::mesh::Model = plane.into();
00007FF6FE0612CC  lea          r14d, [rsi+1h]
00007FF6FE0612D0  mov          eax, esi
00007FF6FE0612D2  mul          sil
00007FF6FE0612D5  mov          ebp, eax
00007FF6FE0612D7  mov          eax, r14d
00007FF6FE0612DA  mul          r14b
00007FF6FE0612DD  movzx        edi, al
00007FF6FE0612E0  test         dil, dil
00007FF6FE0612E3  je           static void Fabled_Engine::main+11Ah (00007FF6FE06132Ah)
00007FF6FE0612E5  mov          rbx, rdi
00007FF6FE0612E8  shl          rbx, 6h
00007FF6FE0612EC  mov          rcx, qword ptr [00007FF6FE0831C8h]
00007FF6FE0612F3  test         rcx, rcx
00007FF6FE0612F6  jne          static void Fabled_Engine::main+FCh (00007FF6FE06130Ch)
00007FF6FE0612F8  call         GetProcessHeap (00007FF6FE07916Ch)
00007FF6FE0612FD  test         rax, rax
00007FF6FE061300  je           static void Fabled_Engine::main+10Bh (00007FF6FE06131Bh)
00007FF6FE061302  mov          rcx, rax
00007FF6FE061305  mov          qword ptr [00007FF6FE0831C8h], rax
00007FF6FE06130C  xor          edx, edx
00007FF6FE06130E  mov          r8, rbx
00007FF6FE061311  call         HeapAlloc (00007FF6FE079172h)
00007FF6FE061316  test         rax, rax
00007FF6FE061319  jne          static void Fabled_Engine::main+11Fh (00007FF6FE06132Fh)
00007FF6FE06131B  mov          edx, 10h
00007FF6FE061320  mov          rcx, rbx
00007FF6FE061323  call         static void alloc::alloc::handle_alloc_error (00007FF6FE07A380h)
00007FF6FE061328  ud2
00007FF6FE06132A  mov          eax, 10h
00007FF6FE06132F  movzx        ecx, bpl
00007FF6FE061333  add          ecx, ecx
00007FF6FE061335  lea          ecx, [rcx+rcx*2]
00007FF6FE061338  movzx        ebx, cl
00007FF6FE06133B  mov          qword ptr [rsp+48h], rax
00007FF6FE061340  mov          qword ptr [rsp+50h], rdi
00007FF6FE061345  mov          qword ptr [rsp+58h], 0h
00007FF6FE06134E  test         cl, cl
00007FF6FE061350  je           static void Fabled_Engine::main+18Bh (00007FF6FE06139Bh)
00007FF6FE061352  lea          rbp, [rbx*8]
00007FF6FE06135A  mov          rcx, qword ptr [00007FF6FE0831C8h]
00007FF6FE061361  test         rcx, rcx
00007FF6FE061364  jne          static void Fabled_Engine::main+16Ah (00007FF6FE06137Ah)
00007FF6FE061366  call         GetProcessHeap (00007FF6FE07916Ch)
00007FF6FE06136B  test         rax, rax
00007FF6FE06136E  je           static void Fabled_Engine::main+17Ch (00007FF6FE06138Ch)
00007FF6FE061370  mov          rcx, rax
00007FF6FE061373  mov          qword ptr [00007FF6FE0831C8h], rax
00007FF6FE06137A  xor          edx, edx
00007FF6FE06137C  mov          r8, rbp
00007FF6FE06137F  call         HeapAlloc (00007FF6FE079172h)
00007FF6FE061384  mov          r15, rax
00007FF6FE061387  test         rax, rax
00007FF6FE06138A  jne          static void Fabled_Engine::main+191h (00007FF6FE0613A1h)
00007FF6FE06138C  mov          edx, 8h
00007FF6FE061391  mov          rcx, rbp
00007FF6FE061394  call         static void alloc::alloc::handle_alloc_error (00007FF6FE07A380h)
00007FF6FE061399  ud2
00007FF6FE06139B  mov          r15d, 8h
00007FF6FE0613A1  mov          qword ptr [rsp+28h], r15
00007FF6FE0613A6  mov          qword ptr [rsp+30h], rbx
00007FF6FE0613AB  mov          qword ptr [rsp+38h], 0h
00007FF6FE0613B4  test         r14b, r14b
00007FF6FE0613B7  je           static void Fabled_Engine::main+2E3h (00007FF6FE0614F3h)
00007FF6FE0613BD  movzx        eax, sil
00007FF6FE0613C1  xorps        xmm0, xmm0
00007FF6FE0613C4  cvtsi2ss     xmm0, eax
00007FF6FE0613C8  movss        xmm9, dword ptr [__real@3f800000 (00007FF6FE07C518h)]
00007FF6FE0613D1  movaps       xmm13, xmm9
00007FF6FE0613D5  divss        xmm13, xmm0
00007FF6FE0613DA  xorps        xmm0, xmm0
00007FF6FE0613DD  xor          edx, edx
00007FF6FE0613DF  movss        xmm10, dword ptr [__real@bf000000 (00007FF6FE07C51Ch)]
00007FF6FE0613E8  lea          r13, [rsp+48h]
00007FF6FE0613ED  mov          ebp, dword ptr [00007FF6FE07DA48h]
00007FF6FE0613F3  mov          rdi, qword ptr [00007FF6FE07DA40h]
00007FF6FE0613FA  movaps       xmm11, xmmword ptr [00007FF6FE07DA50h]
00007FF6FE061402  movaps       xmm12, xmmword ptr [00007FF6FE07DA60h]
00007FF6FE06140A  xor          r12d, r12d
00007FF6FE06140D  jmp          static void Fabled_Engine::main+216h (00007FF6FE061426h)
00007FF6FE06140F  nop
00007FF6FE061410  addss        xmm0, xmm9
00007FF6FE061415  lea          eax, [r12+1h]
00007FF6FE06141A  cmp          r12b, sil
00007FF6FE06141D  mov          r12d, eax
00007FF6FE061420  je           static void Fabled_Engine::main+2DAh (00007FF6FE0614EAh)
00007FF6FE061426  movaps       xmm15, xmm13
00007FF6FE06142A  mulss        xmm15, xmm0
00007FF6FE06142F  movaps       xmm14, xmm15
00007FF6FE061433  addss        xmm14, xmm10
00007FF6FE061438  mov          bl, FFh
00007FF6FE06143A  xorps        xmm6, xmm6
00007FF6FE06143D  jmp          static void Fabled_Engine::main+28Bh (00007FF6FE06149Bh)
00007FF6FE06143F  nop
00007FF6FE061440  mov          rcx, rdx
00007FF6FE061443  mov          rdx, qword ptr [rsp+48h]
00007FF6FE061448  shl          rcx, 6h
00007FF6FE06144C  movss        dword ptr [rdx+rcx*1], xmm8
00007FF6FE061452  mov          dword ptr [rdx+rcx*1+4h], 0h
00007FF6FE06145A  movss        dword ptr [rdx+rcx*1+8h], xmm14
00007FF6FE061461  movss        dword ptr [rdx+rcx*1+Ch], xmm7
00007FF6FE061467  movss        dword ptr [rdx+rcx*1+10h], xmm15
00007FF6FE06146E  mov          dword ptr [rdx+rcx*1+1Ch], ebp
00007FF6FE061472  mov          qword ptr [rdx+rcx*1+14h], rdi
00007FF6FE061477  movaps       xmmword ptr [rdx+rcx*1+20h], xmm11
00007FF6FE06147D  movaps       xmmword ptr [rdx+rcx*1+30h], xmm12
00007FF6FE061483  mov          rdx, rax
00007FF6FE061486  add          rdx, 1h
00007FF6FE06148A  mov          qword ptr [rsp+58h], rdx
00007FF6FE06148F  add          bl, 1h
00007FF6FE061492  cmp          sil, bl
00007FF6FE061495  je           static void Fabled_Engine::main+200h (00007FF6FE061410h)
00007FF6FE06149B  movaps       xmm7, xmm13
00007FF6FE06149F  mulss        xmm7, xmm6
00007FF6FE0614A3  addss        xmm6, xmm9
00007FF6FE0614A8  movaps       xmm8, xmm7
00007FF6FE0614AC  addss        xmm8, xmm10
00007FF6FE0614B1  mov          rax, qword ptr [rsp+58h]
00007FF6FE0614B6  cmp          rdx, qword ptr [rsp+50h]
00007FF6FE0614BB  jne          static void Fabled_Engine::main+230h (00007FF6FE061440h)
00007FF6FE0614BD  mov          rcx, rax
00007FF6FE0614C0  cmp          rdx, rax
00007FF6FE0614C3  jne          static void Fabled_Engine::main+233h (00007FF6FE061443h)
00007FF6FE0614C9  mov          rcx, r13
00007FF6FE0614CC  movss        dword ptr [rsp+60h], xmm0
00007FF6FE0614D2  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF6FE07ACA0h)
00007FF6FE0614D7  movss        xmm0, dword ptr [rsp+60h]
00007FF6FE0614DD  mov          rax, qword ptr [rsp+58h]
00007FF6FE0614E2  mov          rcx, rax
00007FF6FE0614E5  jmp          static void Fabled_Engine::main+233h (00007FF6FE061443h)
00007FF6FE0614EA  test         sil, sil
00007FF6FE0614ED  je           static void Fabled_Engine::main+48Dh (00007FF6FE06169Dh)
00007FF6FE0614F3  movzx        eax, sil
00007FF6FE0614F7  mov          qword ptr [rsp+98h], rax
00007FF6FE0614FF  xor          ebx, ebx
00007FF6FE061501  xor          edx, edx
00007FF6FE061503  xor          eax, eax
00007FF6FE061505  jmp          static void Fabled_Engine::main+311h (00007FF6FE061521h)
00007FF6FE061507  nop          word ptr [rax+rax*1], ax
00007FF6FE061510  add          bl, r14b
00007FF6FE061513  mov          rax, qword ptr [rsp+60h]
00007FF6FE061518  cmp          al, sil
00007FF6FE06151B  je           static void Fabled_Engine::main+48Dh (00007FF6FE06169Dh)
00007FF6FE061521  add          al, 1h
00007FF6FE061523  mov          qword ptr [rsp+60h], rax
00007FF6FE061528  test         sil, sil
00007FF6FE06152B  je           static void Fabled_Engine::main+300h (00007FF6FE061510h)
00007FF6FE06152D  mov          rbp, qword ptr [rsp+98h]
00007FF6FE061535  mov          r13d, ebx
00007FF6FE061538  jmp          static void Fabled_Engine::main+347h (00007FF6FE061557h)
00007FF6FE06153A  nop          word ptr [rax+rax*1], ax
00007FF6FE061540  mov          qword ptr [r15+rdx*8], r12
00007FF6FE061544  add          rdx, 1h
00007FF6FE061548  mov          qword ptr [rsp+38h], rdx
00007FF6FE06154D  add          r13b, 1h
00007FF6FE061551  add          rbp, FFFFFFFFFFFFFFFFh
00007FF6FE061555  je           static void Fabled_Engine::main+300h (00007FF6FE061510h)
00007FF6FE061557  cmp          rdx, qword ptr [rsp+30h]
00007FF6FE06155C  je           static void Fabled_Engine::main+41Ah (00007FF6FE06162Ah)
00007FF6FE061562  lea          edi, [r14+r13*1]
00007FF6FE061566  movzx        r12d, r13b
00007FF6FE06156A  mov          qword ptr [r15+rdx*8], r12
00007FF6FE06156E  mov          rdx, qword ptr [rsp+38h]
00007FF6FE061573  add          rdx, 1h
00007FF6FE061577  mov          qword ptr [rsp+38h], rdx
00007FF6FE06157C  cmp          rdx, qword ptr [rsp+30h]
00007FF6FE061581  je           static void Fabled_Engine::main+433h (00007FF6FE061643h)
00007FF6FE061587  movzx        edi, dil
00007FF6FE06158B  mov          rax, qword ptr [rsp+28h]
00007FF6FE061590  mov          qword ptr [rax+rdx*8], rdi
00007FF6FE061594  mov          rdx, qword ptr [rsp+38h]
00007FF6FE061599  add          rdx, 1h
00007FF6FE06159D  mov          qword ptr [rsp+38h], rdx
00007FF6FE0615A2  add          r12, 1h
00007FF6FE0615A6  cmp          rdx, qword ptr [rsp+30h]
00007FF6FE0615AB  je           static void Fabled_Engine::main+447h (00007FF6FE061657h)
00007FF6FE0615B1  mov          qword ptr [rax+rdx*8], r12
00007FF6FE0615B5  mov          rdx, qword ptr [rsp+38h]
00007FF6FE0615BA  add          rdx, 1h
00007FF6FE0615BE  mov          qword ptr [rsp+38h], rdx
00007FF6FE0615C3  cmp          rdx, qword ptr [rsp+30h]
00007FF6FE0615C8  je           static void Fabled_Engine::main+460h (00007FF6FE061670h)
00007FF6FE0615CE  mov          qword ptr [rax+rdx*8], rdi
00007FF6FE0615D2  mov          rdx, qword ptr [rsp+38h]
00007FF6FE0615D7  add          rdx, 1h
00007FF6FE0615DB  mov          qword ptr [rsp+38h], rdx
00007FF6FE0615E0  add          rdi, 1h
00007FF6FE0615E4  cmp          rdx, qword ptr [rsp+30h]
00007FF6FE0615E9  je           static void Fabled_Engine::main+479h (00007FF6FE061689h)
00007FF6FE0615EF  mov          r15, qword ptr [rsp+28h]
00007FF6FE0615F4  mov          qword ptr [r15+rdx*8], rdi
00007FF6FE0615F8  mov          rdx, qword ptr [rsp+38h]
00007FF6FE0615FD  add          rdx, 1h
00007FF6FE061601  mov          qword ptr [rsp+38h], rdx
00007FF6FE061606  cmp          rdx, qword ptr [rsp+30h]
00007FF6FE06160B  jne          static void Fabled_Engine::main+330h (00007FF6FE061540h)
00007FF6FE061611  lea          rcx, [rsp+28h]
00007FF6FE061616  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6FE07AD60h)
00007FF6FE06161B  mov          r15, qword ptr [rsp+28h]
00007FF6FE061620  mov          rdx, qword ptr [rsp+38h]
00007FF6FE061625  jmp          static void Fabled_Engine::main+330h (00007FF6FE061540h)
00007FF6FE06162A  lea          rcx, [rsp+28h]
00007FF6FE06162F  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6FE07AD60h)
00007FF6FE061634  mov          r15, qword ptr [rsp+28h]
00007FF6FE061639  mov          rdx, qword ptr [rsp+38h]
00007FF6FE06163E  jmp          static void Fabled_Engine::main+352h (00007FF6FE061562h)
00007FF6FE061643  lea          rcx, [rsp+28h]
00007FF6FE061648  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6FE07AD60h)
00007FF6FE06164D  mov          rdx, qword ptr [rsp+38h]
00007FF6FE061652  jmp          static void Fabled_Engine::main+377h (00007FF6FE061587h)
00007FF6FE061657  lea          rcx, [rsp+28h]
00007FF6FE06165C  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6FE07AD60h)
00007FF6FE061661  mov          rax, qword ptr [rsp+28h]
00007FF6FE061666  mov          rdx, qword ptr [rsp+38h]
00007FF6FE06166B  jmp          static void Fabled_Engine::main+3A1h (00007FF6FE0615B1h)
00007FF6FE061670  lea          rcx, [rsp+28h]
00007FF6FE061675  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6FE07AD60h)
00007FF6FE06167A  mov          rax, qword ptr [rsp+28h]
00007FF6FE06167F  mov          rdx, qword ptr [rsp+38h]
00007FF6FE061684  jmp          static void Fabled_Engine::main+3BEh (00007FF6FE0615CEh)
00007FF6FE061689  lea          rcx, [rsp+28h]
00007FF6FE06168E  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6FE07AD60h)
00007FF6FE061693  mov          rdx, qword ptr [rsp+38h]
00007FF6FE061698  jmp          static void Fabled_Engine::main+3DFh (00007FF6FE0615EFh)
00007FF6FE06169D  mov          rax, qword ptr [rsp+38h]
00007FF6FE0616A2  mov          qword ptr [rsp+7Ch], rax
00007FF6FE0616A7  movups       xmm0, xmmword ptr [rsp+28h]
00007FF6FE0616AC  movups       xmmword ptr [rsp+6Ch], xmm0
00007FF6FE0616B1  mov          rcx, qword ptr [00007FF6FE0831C8h]
00007FF6FE0616B8  test         rcx, rcx
00007FF6FE0616BB  jne          static void Fabled_Engine::main+4C5h (00007FF6FE0616D5h)
00007FF6FE0616BD  call         GetProcessHeap (00007FF6FE07916Ch)
00007FF6FE0616C2  test         rax, rax
00007FF6FE0616C5  je           static void Fabled_Engine::main+7B3h (00007FF6FE0619C3h)
00007FF6FE0616CB  mov          rcx, rax
00007FF6FE0616CE  mov          qword ptr [00007FF6FE0831C8h], rax
00007FF6FE0616D5  mov          r8d, 40h
00007FF6FE0616DB  xor          edx, edx
00007FF6FE0616DD  call         HeapAlloc (00007FF6FE079172h)
00007FF6FE0616E2  test         rax, rax
00007FF6FE0616E5  je           static void Fabled_Engine::main+7B3h (00007FF6FE0619C3h)
00007FF6FE0616EB  mov          rsi, rax
00007FF6FE0616EE  mov          rax, qword ptr [rsp+58h]
00007FF6FE0616F3  mov          qword ptr [rsi+10h], rax
00007FF6FE0616F7  movups       xmm0, xmmword ptr [rsp+48h]
00007FF6FE0616FC  movups       xmmword ptr [rsi], xmm0
00007FF6FE0616FF  movups       xmm0, xmmword ptr [rsp+68h]
00007FF6FE061704  movups       xmm1, xmmword ptr [rsp+74h]
00007FF6FE061709  movups       xmmword ptr [rsi+1Ch], xmm0
00007FF6FE06170D  movups       xmmword ptr [rsi+28h], xmm1
00007FF6FE061711  mov          rax, rsi
00007FF6FE061714  add          rax, 18h
00007FF6FE061718  mov          dword ptr [rsi+18h], 0h
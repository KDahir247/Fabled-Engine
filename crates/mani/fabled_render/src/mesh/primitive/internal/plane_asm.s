# --------------- Cube Dissassembly -------------------
//    let plane = fabled_render::mesh::Plane::default();
00007FF65A9012A9  movss        xmm0, dword ptr [__real@41200000 (00007FF65A91C510h)]
00007FF65A9012B1  minss        xmm0, xmm0
00007FF65A9012B5  xorps        xmm1, xmm1
00007FF65A9012B8  maxss        xmm1, xmm0
00007FF65A9012BC  movss        xmm0, dword ptr [__real@437f0000 (00007FF65A91C514h)]
00007FF65A9012C4  minss        xmm0, xmm1
00007FF65A9012C8  cvttss2si    esi, xmm0

//    let plane_model: fabled_render::mesh::Model = plane.into();
00007FF65A9012CC  lea          r14d, [rsi+1h]
00007FF65A9012D0  mov          eax, esi
00007FF65A9012D2  mul          sil
00007FF65A9012D5  mov          ebp, eax
00007FF65A9012D7  mov          eax, r14d
00007FF65A9012DA  mul          r14b
00007FF65A9012DD  movzx        edi, al
00007FF65A9012E0  test         dil, dil
00007FF65A9012E3  je           static void Fabled_Engine::main+11Ah (00007FF65A90132Ah)
00007FF65A9012E5  mov          rbx, rdi
00007FF65A9012E8  shl          rbx, 6h
00007FF65A9012EC  mov          rcx, qword ptr [00007FF65A9231C8h]
00007FF65A9012F3  test         rcx, rcx
00007FF65A9012F6  jne          static void Fabled_Engine::main+FCh (00007FF65A90130Ch)
00007FF65A9012F8  call         GetProcessHeap (00007FF65A91916Ch)
00007FF65A9012FD  test         rax, rax
00007FF65A901300  je           static void Fabled_Engine::main+10Bh (00007FF65A90131Bh)
00007FF65A901302  mov          rcx, rax
00007FF65A901305  mov          qword ptr [00007FF65A9231C8h], rax
00007FF65A90130C  xor          edx, edx
00007FF65A90130E  mov          r8, rbx
00007FF65A901311  call         HeapAlloc (00007FF65A919172h)
00007FF65A901316  test         rax, rax
00007FF65A901319  jne          static void Fabled_Engine::main+11Fh (00007FF65A90132Fh)
00007FF65A90131B  mov          edx, 10h
00007FF65A901320  mov          rcx, rbx
00007FF65A901323  call         static void alloc::alloc::handle_alloc_error (00007FF65A91A380h)
00007FF65A901328  ud2
00007FF65A90132A  mov          eax, 10h
00007FF65A90132F  movzx        ecx, bpl
00007FF65A901333  add          ecx, ecx
00007FF65A901335  lea          ecx, [rcx+rcx*2]
00007FF65A901338  movzx        ebx, cl
00007FF65A90133B  mov          qword ptr [rsp+48h], rax
00007FF65A901340  mov          qword ptr [rsp+50h], rdi
00007FF65A901345  mov          qword ptr [rsp+58h], 0h
00007FF65A90134E  test         cl, cl
00007FF65A901350  je           static void Fabled_Engine::main+18Bh (00007FF65A90139Bh)
00007FF65A901352  lea          rbp, [rbx*8]
00007FF65A90135A  mov          rcx, qword ptr [00007FF65A9231C8h]
00007FF65A901361  test         rcx, rcx
00007FF65A901364  jne          static void Fabled_Engine::main+16Ah (00007FF65A90137Ah)
00007FF65A901366  call         GetProcessHeap (00007FF65A91916Ch)
00007FF65A90136B  test         rax, rax
00007FF65A90136E  je           static void Fabled_Engine::main+17Ch (00007FF65A90138Ch)
00007FF65A901370  mov          rcx, rax
00007FF65A901373  mov          qword ptr [00007FF65A9231C8h], rax
00007FF65A90137A  xor          edx, edx
00007FF65A90137C  mov          r8, rbp
00007FF65A90137F  call         HeapAlloc (00007FF65A919172h)
00007FF65A901384  mov          r15, rax
00007FF65A901387  test         rax, rax
00007FF65A90138A  jne          static void Fabled_Engine::main+191h (00007FF65A9013A1h)
00007FF65A90138C  mov          edx, 8h
00007FF65A901391  mov          rcx, rbp
00007FF65A901394  call         static void alloc::alloc::handle_alloc_error (00007FF65A91A380h)
00007FF65A901399  ud2
00007FF65A90139B  mov          r15d, 8h
00007FF65A9013A1  mov          qword ptr [rsp+28h], r15
00007FF65A9013A6  mov          qword ptr [rsp+30h], rbx
00007FF65A9013AB  mov          qword ptr [rsp+38h], 0h
00007FF65A9013B4  test         r14b, r14b
00007FF65A9013B7  je           static void Fabled_Engine::main+2E3h (00007FF65A9014F3h)
00007FF65A9013BD  movzx        eax, sil
00007FF65A9013C1  xorps        xmm0, xmm0
00007FF65A9013C4  cvtsi2ss     xmm0, eax
00007FF65A9013C8  movss        xmm9, dword ptr [__real@3f800000 (00007FF65A91C518h)]
00007FF65A9013D1  movaps       xmm13, xmm9
00007FF65A9013D5  divss        xmm13, xmm0
00007FF65A9013DA  xorps        xmm0, xmm0
00007FF65A9013DD  xor          edx, edx
00007FF65A9013DF  movss        xmm10, dword ptr [__real@bf000000 (00007FF65A91C51Ch)]
00007FF65A9013E8  lea          r13, [rsp+48h]
00007FF65A9013ED  mov          ebp, dword ptr [00007FF65A91DA48h]
00007FF65A9013F3  mov          rdi, qword ptr [00007FF65A91DA40h]
00007FF65A9013FA  movaps       xmm11, xmmword ptr [00007FF65A91DA50h]
00007FF65A901402  movaps       xmm12, xmmword ptr [00007FF65A91DA60h]
00007FF65A90140A  xor          r12d, r12d
00007FF65A90140D  jmp          static void Fabled_Engine::main+216h (00007FF65A901426h)
00007FF65A90140F  nop
00007FF65A901410  addss        xmm0, xmm9
00007FF65A901415  lea          eax, [r12+1h]
00007FF65A90141A  cmp          r12b, sil
00007FF65A90141D  mov          r12d, eax
00007FF65A901420  je           static void Fabled_Engine::main+2DAh (00007FF65A9014EAh)
00007FF65A901426  movaps       xmm15, xmm13
00007FF65A90142A  mulss        xmm15, xmm0
00007FF65A90142F  movaps       xmm14, xmm15
00007FF65A901433  addss        xmm14, xmm10
00007FF65A901438  mov          bl, FFh
00007FF65A90143A  xorps        xmm6, xmm6
00007FF65A90143D  jmp          static void Fabled_Engine::main+28Bh (00007FF65A90149Bh)
00007FF65A90143F  nop
00007FF65A901440  mov          rcx, rdx
00007FF65A901443  mov          rdx, qword ptr [rsp+48h]
00007FF65A901448  shl          rcx, 6h
00007FF65A90144C  movss        dword ptr [rdx+rcx*1], xmm8
00007FF65A901452  mov          dword ptr [rdx+rcx*1+4h], 0h
00007FF65A90145A  movss        dword ptr [rdx+rcx*1+8h], xmm14
00007FF65A901461  movss        dword ptr [rdx+rcx*1+Ch], xmm7
00007FF65A901467  movss        dword ptr [rdx+rcx*1+10h], xmm15
00007FF65A90146E  mov          dword ptr [rdx+rcx*1+1Ch], ebp
00007FF65A901472  mov          qword ptr [rdx+rcx*1+14h], rdi
00007FF65A901477  movaps       xmmword ptr [rdx+rcx*1+20h], xmm11
00007FF65A90147D  movaps       xmmword ptr [rdx+rcx*1+30h], xmm12
00007FF65A901483  mov          rdx, rax
00007FF65A901486  add          rdx, 1h
00007FF65A90148A  mov          qword ptr [rsp+58h], rdx
00007FF65A90148F  add          bl, 1h
00007FF65A901492  cmp          sil, bl
00007FF65A901495  je           static void Fabled_Engine::main+200h (00007FF65A901410h)
00007FF65A90149B  movaps       xmm7, xmm13
00007FF65A90149F  mulss        xmm7, xmm6
00007FF65A9014A3  addss        xmm6, xmm9
00007FF65A9014A8  movaps       xmm8, xmm7
00007FF65A9014AC  addss        xmm8, xmm10
00007FF65A9014B1  mov          rax, qword ptr [rsp+58h]
00007FF65A9014B6  cmp          rdx, qword ptr [rsp+50h]
00007FF65A9014BB  jne          static void Fabled_Engine::main+230h (00007FF65A901440h)
00007FF65A9014BD  mov          rcx, rax
00007FF65A9014C0  cmp          rdx, rax
00007FF65A9014C3  jne          static void Fabled_Engine::main+233h (00007FF65A901443h)
00007FF65A9014C9  mov          rcx, r13
00007FF65A9014CC  movss        dword ptr [rsp+60h], xmm0
00007FF65A9014D2  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF65A91ACA0h)
00007FF65A9014D7  movss        xmm0, dword ptr [rsp+60h]
00007FF65A9014DD  mov          rax, qword ptr [rsp+58h]
00007FF65A9014E2  mov          rcx, rax
00007FF65A9014E5  jmp          static void Fabled_Engine::main+233h (00007FF65A901443h)
00007FF65A9014EA  test         sil, sil
00007FF65A9014ED  je           static void Fabled_Engine::main+48Dh (00007FF65A90169Dh)
00007FF65A9014F3  movzx        eax, sil
00007FF65A9014F7  mov          qword ptr [rsp+98h], rax
00007FF65A9014FF  xor          ebx, ebx
00007FF65A901501  xor          edx, edx
00007FF65A901503  xor          eax, eax
00007FF65A901505  jmp          static void Fabled_Engine::main+311h (00007FF65A901521h)
00007FF65A901507  nop          word ptr [rax+rax*1], ax
00007FF65A901510  add          bl, r14b
00007FF65A901513  mov          rax, qword ptr [rsp+60h]
00007FF65A901518  cmp          al, sil
00007FF65A90151B  je           static void Fabled_Engine::main+48Dh (00007FF65A90169Dh)
00007FF65A901521  add          al, 1h
00007FF65A901523  mov          qword ptr [rsp+60h], rax
00007FF65A901528  test         sil, sil
00007FF65A90152B  je           static void Fabled_Engine::main+300h (00007FF65A901510h)
00007FF65A90152D  mov          rbp, qword ptr [rsp+98h]
00007FF65A901535  mov          r13d, ebx
00007FF65A901538  jmp          static void Fabled_Engine::main+347h (00007FF65A901557h)
00007FF65A90153A  nop          word ptr [rax+rax*1], ax
00007FF65A901540  mov          qword ptr [r15+rdx*8], r12
00007FF65A901544  add          rdx, 1h
00007FF65A901548  mov          qword ptr [rsp+38h], rdx
00007FF65A90154D  add          r13b, 1h
00007FF65A901551  add          rbp, FFFFFFFFFFFFFFFFh
00007FF65A901555  je           static void Fabled_Engine::main+300h (00007FF65A901510h)
00007FF65A901557  cmp          rdx, qword ptr [rsp+30h]
00007FF65A90155C  je           static void Fabled_Engine::main+41Ah (00007FF65A90162Ah)
00007FF65A901562  lea          edi, [r14+r13*1]
00007FF65A901566  movzx        r12d, r13b
00007FF65A90156A  mov          qword ptr [r15+rdx*8], r12
00007FF65A90156E  mov          rdx, qword ptr [rsp+38h]
00007FF65A901573  add          rdx, 1h
00007FF65A901577  mov          qword ptr [rsp+38h], rdx
00007FF65A90157C  cmp          rdx, qword ptr [rsp+30h]
00007FF65A901581  je           static void Fabled_Engine::main+433h (00007FF65A901643h)
00007FF65A901587  movzx        edi, dil
00007FF65A90158B  mov          rax, qword ptr [rsp+28h]
00007FF65A901590  mov          qword ptr [rax+rdx*8], rdi
00007FF65A901594  mov          rdx, qword ptr [rsp+38h]
00007FF65A901599  add          rdx, 1h
00007FF65A90159D  mov          qword ptr [rsp+38h], rdx
00007FF65A9015A2  add          r12, 1h
00007FF65A9015A6  cmp          rdx, qword ptr [rsp+30h]
00007FF65A9015AB  je           static void Fabled_Engine::main+447h (00007FF65A901657h)
00007FF65A9015B1  mov          qword ptr [rax+rdx*8], r12
00007FF65A9015B5  mov          rdx, qword ptr [rsp+38h]
00007FF65A9015BA  add          rdx, 1h
00007FF65A9015BE  mov          qword ptr [rsp+38h], rdx
00007FF65A9015C3  cmp          rdx, qword ptr [rsp+30h]
00007FF65A9015C8  je           static void Fabled_Engine::main+460h (00007FF65A901670h)
00007FF65A9015CE  mov          qword ptr [rax+rdx*8], rdi
00007FF65A9015D2  mov          rdx, qword ptr [rsp+38h]
00007FF65A9015D7  add          rdx, 1h
00007FF65A9015DB  mov          qword ptr [rsp+38h], rdx
00007FF65A9015E0  add          rdi, 1h
00007FF65A9015E4  cmp          rdx, qword ptr [rsp+30h]
00007FF65A9015E9  je           static void Fabled_Engine::main+479h (00007FF65A901689h)
00007FF65A9015EF  mov          r15, qword ptr [rsp+28h]
00007FF65A9015F4  mov          qword ptr [r15+rdx*8], rdi
00007FF65A9015F8  mov          rdx, qword ptr [rsp+38h]
00007FF65A9015FD  add          rdx, 1h
00007FF65A901601  mov          qword ptr [rsp+38h], rdx
00007FF65A901606  cmp          rdx, qword ptr [rsp+30h]
00007FF65A90160B  jne          static void Fabled_Engine::main+330h (00007FF65A901540h)
00007FF65A901611  lea          rcx, [rsp+28h]
00007FF65A901616  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF65A91AD60h)
00007FF65A90161B  mov          r15, qword ptr [rsp+28h]
00007FF65A901620  mov          rdx, qword ptr [rsp+38h]
00007FF65A901625  jmp          static void Fabled_Engine::main+330h (00007FF65A901540h)
00007FF65A90162A  lea          rcx, [rsp+28h]
00007FF65A90162F  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF65A91AD60h)
00007FF65A901634  mov          r15, qword ptr [rsp+28h]
00007FF65A901639  mov          rdx, qword ptr [rsp+38h]
00007FF65A90163E  jmp          static void Fabled_Engine::main+352h (00007FF65A901562h)
00007FF65A901643  lea          rcx, [rsp+28h]
00007FF65A901648  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF65A91AD60h)
00007FF65A90164D  mov          rdx, qword ptr [rsp+38h]
00007FF65A901652  jmp          static void Fabled_Engine::main+377h (00007FF65A901587h)
00007FF65A901657  lea          rcx, [rsp+28h]
00007FF65A90165C  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF65A91AD60h)
00007FF65A901661  mov          rax, qword ptr [rsp+28h]
00007FF65A901666  mov          rdx, qword ptr [rsp+38h]
00007FF65A90166B  jmp          static void Fabled_Engine::main+3A1h (00007FF65A9015B1h)
00007FF65A901670  lea          rcx, [rsp+28h]
00007FF65A901675  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF65A91AD60h)
00007FF65A90167A  mov          rax, qword ptr [rsp+28h]
00007FF65A90167F  mov          rdx, qword ptr [rsp+38h]
00007FF65A901684  jmp          static void Fabled_Engine::main+3BEh (00007FF65A9015CEh)
00007FF65A901689  lea          rcx, [rsp+28h]
00007FF65A90168E  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF65A91AD60h)
00007FF65A901693  mov          rdx, qword ptr [rsp+38h]
00007FF65A901698  jmp          static void Fabled_Engine::main+3DFh (00007FF65A9015EFh)
00007FF65A90169D  mov          rax, qword ptr [rsp+38h]
00007FF65A9016A2  mov          qword ptr [rsp+7Ch], rax
00007FF65A9016A7  movups       xmm0, xmmword ptr [rsp+28h]
00007FF65A9016AC  movups       xmmword ptr [rsp+6Ch], xmm0
00007FF65A9016B1  mov          rcx, qword ptr [00007FF65A9231C8h]
00007FF65A9016B8  test         rcx, rcx
00007FF65A9016BB  jne          static void Fabled_Engine::main+4C5h (00007FF65A9016D5h)
00007FF65A9016BD  call         GetProcessHeap (00007FF65A91916Ch)
00007FF65A9016C2  test         rax, rax
00007FF65A9016C5  je           static void Fabled_Engine::main+7B3h (00007FF65A9019C3h)
00007FF65A9016CB  mov          rcx, rax
00007FF65A9016CE  mov          qword ptr [00007FF65A9231C8h], rax
00007FF65A9016D5  mov          r8d, 40h
00007FF65A9016DB  xor          edx, edx
00007FF65A9016DD  call         HeapAlloc (00007FF65A919172h)
00007FF65A9016E2  test         rax, rax
00007FF65A9016E5  je           static void Fabled_Engine::main+7B3h (00007FF65A9019C3h)
00007FF65A9016EB  mov          rsi, rax
00007FF65A9016EE  mov          rax, qword ptr [rsp+58h]
00007FF65A9016F3  mov          qword ptr [rsi+10h], rax
00007FF65A9016F7  movups       xmm0, xmmword ptr [rsp+48h]
00007FF65A9016FC  movups       xmmword ptr [rsi], xmm0
00007FF65A9016FF  movups       xmm0, xmmword ptr [rsp+68h]
00007FF65A901704  movups       xmm1, xmmword ptr [rsp+74h]
00007FF65A901709  movups       xmmword ptr [rsi+1Ch], xmm0
00007FF65A90170D  movups       xmmword ptr [rsi+28h], xmm1
00007FF65A901711  mov          rax, rsi
00007FF65A901714  add          rax, 18h
00007FF65A901718  mov          dword ptr [rsi+18h], 0h
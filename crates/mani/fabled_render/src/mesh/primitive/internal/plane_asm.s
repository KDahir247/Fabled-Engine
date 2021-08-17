# --------------- Cube Dissassembly -------------------
//    let plane = fabled_render::mesh::Plane::default();
00007FF615981359  mov          rcx, qword ptr [00007FF6159A31C8h]
00007FF615981360  test         rcx, rcx
00007FF615981363  jne          static void Fabled_Engine::main+BDh (00007FF61598137Dh)
00007FF615981365  call         GetProcessHeap (00007FF615998E92h)
00007FF61598136A  test         rax, rax
00007FF61598136D  je           static void Fabled_Engine::main+6D0h (00007FF615981990h)
00007FF615981373  mov          rcx, rax
00007FF615981376  mov          qword ptr [00007FF6159A31C8h], rax
00007FF61598137D  mov          r8d, 1E40h
00007FF615981383  xor          edx, edx
00007FF615981385  call         HeapAlloc (00007FF615998E98h)
00007FF61598138A  test         rax, rax
00007FF61598138D  je           static void Fabled_Engine::main+6D0h (00007FF615981990h)
00007FF615981393  mov          qword ptr [rsp+48h], rax
00007FF615981398  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000079 (00007FF61599C520h)]
00007FF61598139F  movups       xmmword ptr [rsp+50h], xmm0
00007FF6159813A4  mov          rcx, qword ptr [00007FF6159A31C8h]
00007FF6159813AB  test         rcx, rcx
00007FF6159813AE  jne          static void Fabled_Engine::main+108h (00007FF6159813C8h)
00007FF6159813B0  call         GetProcessHeap (00007FF615998E92h)
00007FF6159813B5  test         rax, rax
00007FF6159813B8  je           static void Fabled_Engine::main+6D7h (00007FF615981997h)
00007FF6159813BE  mov          rcx, rax
00007FF6159813C1  mov          qword ptr [00007FF6159A31C8h], rax
00007FF6159813C8  mov          r8d, 2C0h
00007FF6159813CE  xor          edx, edx
00007FF6159813D0  call         HeapAlloc (00007FF615998E98h)
00007FF6159813D5  test         rax, rax
00007FF6159813D8  je           static void Fabled_Engine::main+6D7h (00007FF615981997h)
00007FF6159813DE  mov          r15, rax
00007FF6159813E1  mov          qword ptr [rsp+28h], rax
00007FF6159813E6  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000058 (00007FF61599C530h)]
00007FF6159813ED  movups       xmmword ptr [rsp+30h], xmm0
00007FF6159813F2  xorps        xmm8, xmm8
00007FF6159813F6  mov          rdx, qword ptr [rsp+58h]
00007FF6159813FB  xor          esi, esi
00007FF6159813FD  movss        xmm9, dword ptr [__real@3dcccccd (00007FF61599C540h)]
00007FF615981406  movss        xmm10, dword ptr [__real@bf000000 (00007FF61599C544h)]
00007FF61598140F  xorps        xmm12, xmm12
00007FF615981413  movss        xmm11, dword ptr [__real@3f800000 (00007FF61599C548h)]
00007FF61598141C  lea          r14, [rsp+48h]
00007FF615981421  mov          edi, dword ptr [00007FF61599DA58h]
00007FF615981427  mov          rbp, qword ptr [00007FF61599DA50h]
00007FF61598142E  jmp          static void Fabled_Engine::main+183h (00007FF615981443h)
00007FF615981430  addss        xmm8, xmm11
00007FF615981435  add          sil, 1h
00007FF615981439  cmp          sil, Bh
00007FF61598143D  je           static void Fabled_Engine::main+241h (00007FF615981501h)
00007FF615981443  movaps       xmm14, xmm8
00007FF615981447  mulss        xmm14, xmm9
00007FF61598144C  movaps       xmm13, xmm14
00007FF615981450  addss        xmm13, xmm10
00007FF615981455  mov          bl, Bh
00007FF615981457  xorps        xmm15, xmm15
00007FF61598145B  jmp          static void Fabled_Engine::main+1FFh (00007FF6159814BFh)
00007FF61598145D  nop          dword ptr [rax], eax
00007FF615981460  mov          rax, qword ptr [rsp+48h]
00007FF615981465  mov          rcx, rdx
00007FF615981468  shl          rcx, 6h
00007FF61598146C  movss        dword ptr [rax+rcx*1], xmm6
00007FF615981471  mov          dword ptr [rax+rcx*1+4h], 0h
00007FF615981479  movss        dword ptr [rax+rcx*1+8h], xmm13
00007FF615981480  movss        dword ptr [rax+rcx*1+Ch], xmm7
00007FF615981486  movss        dword ptr [rax+rcx*1+10h], xmm14
00007FF61598148D  mov          dword ptr [rax+rcx*1+1Ch], edi
00007FF615981491  mov          qword ptr [rax+rcx*1+14h], rbp
00007FF615981496  movaps       xmm0, xmmword ptr [rsp+60h]
00007FF61598149B  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF6159814A0  movaps       xmm0, xmmword ptr [rsp+90h]
00007FF6159814A8  movaps       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF6159814AD  add          rdx, 1h
00007FF6159814B1  mov          qword ptr [rsp+58h], rdx
00007FF6159814B6  add          bl, FFh
00007FF6159814B9  je           static void Fabled_Engine::main+170h (00007FF615981430h)
00007FF6159814BF  movaps       xmm7, xmm15
00007FF6159814C3  mulss        xmm7, xmm9
00007FF6159814C8  movaps       xmm6, xmm7
00007FF6159814CB  addss        xmm6, xmm10
00007FF6159814D0  movaps       xmmword ptr [rsp+60h], xmm12
00007FF6159814D6  addss        xmm15, xmm11
00007FF6159814DB  movaps       xmmword ptr [rsp+90h], xmm12
00007FF6159814E4  cmp          rdx, qword ptr [rsp+50h]
00007FF6159814E9  jne          static void Fabled_Engine::main+1A0h (00007FF615981460h)
00007FF6159814EF  mov          rcx, r14
00007FF6159814F2  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF61599A9C0h)
00007FF6159814F7  mov          rdx, qword ptr [rsp+58h]
00007FF6159814FC  jmp          static void Fabled_Engine::main+1A0h (00007FF615981460h)
00007FF615981501  mov          r13d, Ch
00007FF615981507  xor          edx, edx
00007FF615981509  lea          r12, [rsp+28h]
00007FF61598150E  xor          ebx, ebx
00007FF615981510  jmp          static void Fabled_Engine::main+26Dh (00007FF61598152Dh)
00007FF615981512  nop          word ptr cs:[rax+rax*1], ax
00007FF61598151C  nop          dword ptr [rax], eax
00007FF615981520  add          r13, Bh
00007FF615981524  cmp          bl, Ah
00007FF615981527  je           static void Fabled_Engine::main+3C9h (00007FF615981689h)
00007FF61598152D  add          bl, 1h
00007FF615981530  mov          edi, Ah
00007FF615981535  mov          rsi, r13
00007FF615981538  jmp          static void Fabled_Engine::main+297h (00007FF615981557h)
00007FF61598153A  nop          word ptr [rax+rax*1], ax
00007FF615981540  mov          qword ptr [r15+rdx*8], rbp
00007FF615981544  add          rdx, 1h
00007FF615981548  mov          qword ptr [rsp+38h], rdx
00007FF61598154D  add          rsi, 1h
00007FF615981551  add          rdi, FFFFFFFFFFFFFFFFh
00007FF615981555  je           static void Fabled_Engine::main+260h (00007FF615981520h)
00007FF615981557  lea          rbp, [rsi-Ch]
00007FF61598155B  cmp          rdx, qword ptr [rsp+30h]
00007FF615981560  je           static void Fabled_Engine::main+360h (00007FF615981620h)
00007FF615981566  lea          r14, [rsi-1h]
00007FF61598156A  mov          qword ptr [r15+rdx*8], rbp
00007FF61598156E  mov          rdx, qword ptr [rsp+38h]
00007FF615981573  add          rdx, 1h
00007FF615981577  mov          qword ptr [rsp+38h], rdx
00007FF61598157C  cmp          rdx, qword ptr [rsp+30h]
00007FF615981581  je           static void Fabled_Engine::main+377h (00007FF615981637h)
00007FF615981587  mov          rax, qword ptr [rsp+28h]
00007FF61598158C  mov          qword ptr [rax+rdx*8], r14
00007FF615981590  mov          rdx, qword ptr [rsp+38h]
00007FF615981595  add          rdx, 1h
00007FF615981599  mov          qword ptr [rsp+38h], rdx
00007FF61598159E  lea          rbp, [rsi-Bh]
00007FF6159815A2  cmp          rdx, qword ptr [rsp+30h]
00007FF6159815A7  je           static void Fabled_Engine::main+389h (00007FF615981649h)
00007FF6159815AD  mov          qword ptr [rax+rdx*8], rbp
00007FF6159815B1  mov          rdx, qword ptr [rsp+38h]
00007FF6159815B6  add          rdx, 1h
00007FF6159815BA  mov          qword ptr [rsp+38h], rdx
00007FF6159815BF  cmp          rdx, qword ptr [rsp+30h]
00007FF6159815C4  je           static void Fabled_Engine::main+3A0h (00007FF615981660h)
00007FF6159815CA  mov          qword ptr [rax+rdx*8], r14
00007FF6159815CE  mov          rdx, qword ptr [rsp+38h]
00007FF6159815D3  add          rdx, 1h
00007FF6159815D7  mov          qword ptr [rsp+38h], rdx
00007FF6159815DC  cmp          rdx, qword ptr [rsp+30h]
00007FF6159815E1  je           static void Fabled_Engine::main+3B7h (00007FF615981677h)
00007FF6159815E7  mov          r15, qword ptr [rsp+28h]
00007FF6159815EC  mov          qword ptr [r15+rdx*8], rsi
00007FF6159815F0  mov          rdx, qword ptr [rsp+38h]
00007FF6159815F5  add          rdx, 1h
00007FF6159815F9  mov          qword ptr [rsp+38h], rdx
00007FF6159815FE  cmp          rdx, qword ptr [rsp+30h]
00007FF615981603  jne          static void Fabled_Engine::main+280h (00007FF615981540h)
00007FF615981609  mov          rcx, r12
00007FF61598160C  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF61599AA80h)
00007FF615981611  mov          r15, qword ptr [rsp+28h]
00007FF615981616  mov          rdx, qword ptr [rsp+38h]
00007FF61598161B  jmp          static void Fabled_Engine::main+280h (00007FF615981540h)
00007FF615981620  mov          rcx, r12
00007FF615981623  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF61599AA80h)
00007FF615981628  mov          r15, qword ptr [rsp+28h]
00007FF61598162D  mov          rdx, qword ptr [rsp+38h]
00007FF615981632  jmp          static void Fabled_Engine::main+2A6h (00007FF615981566h)
00007FF615981637  mov          rcx, r12
00007FF61598163A  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF61599AA80h)
00007FF61598163F  mov          rdx, qword ptr [rsp+38h]
00007FF615981644  jmp          static void Fabled_Engine::main+2C7h (00007FF615981587h)
00007FF615981649  mov          rcx, r12
00007FF61598164C  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF61599AA80h)
00007FF615981651  mov          rax, qword ptr [rsp+28h]
00007FF615981656  mov          rdx, qword ptr [rsp+38h]
00007FF61598165B  jmp          static void Fabled_Engine::main+2EDh (00007FF6159815ADh)
00007FF615981660  mov          rcx, r12
00007FF615981663  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF61599AA80h)
00007FF615981668  mov          rax, qword ptr [rsp+28h]
00007FF61598166D  mov          rdx, qword ptr [rsp+38h]
00007FF615981672  jmp          static void Fabled_Engine::main+30Ah (00007FF6159815CAh)
00007FF615981677  mov          rcx, r12
00007FF61598167A  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF61599AA80h)
00007FF61598167F  mov          rdx, qword ptr [rsp+38h]
00007FF615981684  jmp          static void Fabled_Engine::main+327h (00007FF6159815E7h)
00007FF615981689  mov          rax, qword ptr [rsp+38h]
00007FF61598168E  mov          qword ptr [rsp+74h], rax
00007FF615981693  movups       xmm0, xmmword ptr [rsp+28h]
00007FF615981698  movups       xmmword ptr [rsp+64h], xmm0
00007FF61598169D  mov          rcx, qword ptr [00007FF6159A31C8h]
00007FF6159816A4  test         rcx, rcx
00007FF6159816A7  je           static void Fabled_Engine::main+3F0h (00007FF6159816B0h)
00007FF6159816A9  lea          r13, [rsp+48h]
00007FF6159816AE  jmp          static void Fabled_Engine::main+40Dh (00007FF6159816CDh)
00007FF6159816B0  call         GetProcessHeap (00007FF615998E92h)
00007FF6159816B5  test         rax, rax
00007FF6159816B8  lea          r13, [rsp+48h]
00007FF6159816BD  je           static void Fabled_Engine::main+6E8h (00007FF6159819A8h)
00007FF6159816C3  mov          rcx, rax
00007FF6159816C6  mov          qword ptr [00007FF6159A31C8h], rax
00007FF6159816CD  mov          r8d, 40h
00007FF6159816D3  xor          edx, edx
00007FF6159816D5  call         HeapAlloc (00007FF615998E98h)
00007FF6159816DA  test         rax, rax
00007FF6159816DD  je           static void Fabled_Engine::main+6E8h (00007FF6159819A8h)
00007FF6159816E3  mov          rbx, rax
00007FF6159816E6  mov          rax, qword ptr [rsp+58h]
00007FF6159816EB  mov          qword ptr [rbx+10h], rax
00007FF6159816EF  movups       xmm0, xmmword ptr [rsp+48h]
00007FF6159816F4  movups       xmmword ptr [rbx], xmm0
00007FF6159816F7  movups       xmm0, xmmword ptr [rsp+60h]
00007FF6159816FC  movups       xmm1, xmmword ptr [rsp+6Ch]
00007FF615981701  movups       xmmword ptr [rbx+1Ch], xmm0
00007FF615981705  movups       xmmword ptr [rbx+28h], xmm1
00007FF615981709  mov          dword ptr [rbx+18h], 0h
00007FF615981710  lea          rax, [00007FF61599FAA0h]
00007FF615981717  mov          qword ptr [rsp+90h], rax
00007FF61598171F  mov          qword ptr [rsp+98h], 6h
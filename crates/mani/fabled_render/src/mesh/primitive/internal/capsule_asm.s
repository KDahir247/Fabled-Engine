# --------------- Capsule Dissassembly -------------------
//   let capsule = fabled_render::mesh::Capsule::default();
00007FF69FC212A9  movss        xmm0, dword ptr [__real@41000000 (00007FF69FC3E520h)]
00007FF69FC212B1  minss        xmm0, dword ptr [__real@42b40000 (00007FF69FC3E524h)]
00007FF69FC212B9  xorps        xmm1, xmm1
00007FF69FC212BC  xorps        xmm2, xmm2
00007FF69FC212BF  maxss        xmm2, xmm0
00007FF69FC212C3  movss        xmm0, dword ptr [__real@437f0000 (00007FF69FC3E528h)]
00007FF69FC212CB  movaps       xmm3, xmm0
00007FF69FC212CE  minss        xmm3, xmm2
00007FF69FC212D2  cvttss2si    r14d, xmm3
00007FF69FC212D7  movss        xmm2, dword ptr [__real@41800000 (00007FF69FC3E52Ch)]
00007FF69FC212DF  minss        xmm2, dword ptr [__real@43340000 (00007FF69FC3E530h)]
00007FF69FC212E7  xorps        xmm3, xmm3
00007FF69FC212EA  maxss        xmm3, xmm2
00007FF69FC212EE  movaps       xmm2, xmm0
00007FF69FC212F1  minss        xmm2, xmm3
00007FF69FC212F5  cvttss2si    eax, xmm2
00007FF69FC212F9  movss        xmm12, dword ptr [__real@3f800000 (00007FF69FC3E534h)]
00007FF69FC21302  movaps       xmm2, xmm12
00007FF69FC21306  minss        xmm2, dword ptr [__real@40a00000 (00007FF69FC3E538h)]
00007FF69FC2130E  maxss        xmm1, xmm2
00007FF69FC21312  minss        xmm0, xmm1
00007FF69FC21316  cvttss2si    r15d, xmm0
00007FF69FC2131B  movzx        r13d, r15b
00007FF69FC2131F  mov          qword ptr [rsp+48h], rax
00007FF69FC21324  movzx        ebx, al


//  let capsule_model: fabled_render::mesh::Model = capsule.into();
00007FF69FC21327  mov          eax, r14d
00007FF69FC2132A  shr          al, 1h
00007FF69FC2132C  movzx        esi, al
00007FF69FC2132F  lea          rdi, [rsi-1h]
00007FF69FC21333  lea          rax, [rsi-2h]
00007FF69FC21337  lea          rdx, [rbx+1h]
00007FF69FC2133B  mov          qword ptr [rsp+78h], rdi
00007FF69FC21340  imul         rdi, rdx
00007FF69FC21344  lea          r12, [rdi+rbx*1]
00007FF69FC21348  lea          rbp, [rbx+r12*1]
00007FF69FC2134C  add          rbp, 1h
00007FF69FC21350  mov          rcx, rdx
00007FF69FC21353  imul         rcx, r13
00007FF69FC21357  mov          qword ptr [rsp+170h], rbp
00007FF69FC2135F  add          rcx, rbp
00007FF69FC21362  add          rcx, rbx
00007FF69FC21365  add          rcx, 1h
00007FF69FC21369  mov          qword ptr [rsp+58h], rdx
00007FF69FC2136E  imul         rax, rdx
00007FF69FC21372  add          rax, rcx
00007FF69FC21375  lea          rbp, [rbx+rax*1]
00007FF69FC21379  add          rbp, 1h
00007FF69FC2137D  add          rbp, rbx
00007FF69FC21380  xorps        xmm6, xmm6
00007FF69FC21383  movaps       xmmword ptr [rsp+250h], xmm6
00007FF69FC2138B  lea          rcx, [rsp+188h]
00007FF69FC21393  lea          rdx, [rsp+250h]
00007FF69FC2139B  mov          r8, rbp
00007FF69FC2139E  call         static struct alloc::vec::Vec<glam::vec3::Vec3A, alloc::alloc::Global> alloc::vec::from_elem<glam::vec3::Vec3A> (00007FF69FC272A0h)
00007FF69FC213A3  lea          rcx, [rsp+D0h]
00007FF69FC213AB  mov          rdx, rbp
00007FF69FC213AE  call         static struct alloc::vec::Vec<glam::vec2::Vec2, alloc::alloc::Global> alloc::vec::from_elem<glam::vec2::Vec2> (00007FF69FC27160h)
00007FF69FC213B3  movaps       xmmword ptr [rsp+240h], xmm6
00007FF69FC213BB  lea          rcx, [rsp+E8h]
00007FF69FC213C3  lea          rdx, [rsp+240h]
00007FF69FC213CB  mov          r8, rbp
00007FF69FC213CE  call         static struct alloc::vec::Vec<glam::vec3::Vec3A, alloc::alloc::Global> alloc::vec::from_elem<glam::vec3::Vec3A> (00007FF69FC272A0h)
00007FF69FC213D3  lea          rcx, [rsp+60h]
00007FF69FC213D8  mov          rdx, rbx
00007FF69FC213DB  call         static struct alloc::vec::Vec<glam::vec2::Vec2, alloc::alloc::Global> alloc::vec::from_elem<glam::vec2::Vec2> (00007FF69FC27160h)
00007FF69FC213E0  lea          rcx, [rsp+90h]
00007FF69FC213E8  mov          rdx, rbx
00007FF69FC213EB  call         static struct alloc::vec::Vec<glam::vec2::Vec2, alloc::alloc::Global> alloc::vec::from_elem<glam::vec2::Vec2> (00007FF69FC27160h)
00007FF69FC213F0  mov          qword ptr [rsp+28h], rbx
00007FF69FC213F5  lea          r8, [rbx*4+4h]
00007FF69FC213FD  mov          rcx, qword ptr [00007FF69FC451C8h]
00007FF69FC21404  test         rcx, rcx
00007FF69FC21407  mov          qword ptr [rsp+80h], r8
00007FF69FC2140F  jne          static void Fabled_Engine::main+221h (00007FF69FC21431h)
00007FF69FC21411  call         GetProcessHeap (00007FF69FC3AA6Ch)
00007FF69FC21416  test         rax, rax
00007FF69FC21419  je           static void Fabled_Engine::main+1D99h (00007FF69FC22FA9h)
00007FF69FC2141F  mov          rcx, rax
00007FF69FC21422  mov          qword ptr [00007FF69FC451C8h], rax
00007FF69FC21429  mov          r8, qword ptr [rsp+80h]
00007FF69FC21431  mov          edx, 8h
00007FF69FC21436  call         HeapAlloc (00007FF69FC3AA72h)
00007FF69FC2143B  mov          qword ptr [rsp+88h], rax
00007FF69FC21443  test         rax, rax
00007FF69FC21446  je           static void Fabled_Engine::main+1D99h (00007FF69FC22FA9h)
00007FF69FC2144C  mov          qword ptr [rsp+1D8h], rdi
00007FF69FC21454  mov          qword ptr [rsp+1A0h], r12
00007FF69FC2145C  mov          qword ptr [rsp+1D0h], rbp
00007FF69FC21464  movzx        eax, byte ptr [rsp+48h]
00007FF69FC21469  xorps        xmm0, xmm0
00007FF69FC2146C  cvtsi2ss     xmm0, eax
00007FF69FC21470  movaps       xmm14, xmm12
00007FF69FC21474  divss        xmm14, xmm0
00007FF69FC21479  test         al, al
00007FF69FC2147B  mov          rcx, qword ptr [rsp+28h]
00007FF69FC21480  je           static void Fabled_Engine::main+1DADh (00007FF69FC22FBDh)
00007FF69FC21486  movzx        eax, r15b
00007FF69FC2148A  xorps        xmm0, xmm0
00007FF69FC2148D  cvtsi2ss     xmm0, eax
00007FF69FC21491  minss        xmm0, xmm12
00007FF69FC21496  movaps       xmmword ptr [rsp+220h], xmm0
00007FF69FC2149E  lea          rax, [r13+1h]
00007FF69FC214A2  mov          qword ptr [rsp+C0h], rax
00007FF69FC214AA  movzx        eax, r14b
00007FF69FC214AE  xorps        xmm0, xmm0
00007FF69FC214B1  cvtsi2ss     xmm0, eax
00007FF69FC214B5  movaps       xmm1, xmm12
00007FF69FC214B9  divss        xmm1, xmm0
00007FF69FC214BD  movss        xmm10, dword ptr [__real@40c90fdb (00007FF69FC3E53Ch)]
00007FF69FC214C6  mulss        xmm10, xmm14
00007FF69FC214CB  movss        xmm0, dword ptr [__real@40490fdb (00007FF69FC3E540h)]
00007FF69FC214D3  mulss        xmm0, xmm1
00007FF69FC214D7  movss        dword ptr [rsp+184h], xmm0
00007FF69FC214E0  addss        xmm1, xmm1
00007FF69FC214E4  movaps       xmmword ptr [rsp+230h], xmm1
00007FF69FC214EC  mov          rax, qword ptr [rsp+60h]
00007FF69FC214F1  mov          qword ptr [rsp+30h], rax
00007FF69FC214F6  mov          rax, qword ptr [rsp+70h]
00007FF69FC214FB  mov          qword ptr [rsp+38h], rax
00007FF69FC21500  mov          rax, qword ptr [rsp+90h]
00007FF69FC21508  mov          qword ptr [rsp+130h], rax
00007FF69FC21510  mov          rax, qword ptr [rsp+A0h]
00007FF69FC21518  mov          qword ptr [rsp+B0h], rax
00007FF69FC21520  mov          rbx, qword ptr [rsp+D0h]
00007FF69FC21528  mov          rbp, qword ptr [rsp+E0h]
00007FF69FC21530  mov          rax, r13
00007FF69FC21533  mov          qword ptr [rsp+160h], r13
00007FF69FC2153B  mov          qword ptr [rsp+1A8h], rsi
00007FF69FC21543  lea          rax, [rsi*2-3h]
00007FF69FC2154B  add          rax, r13
00007FF69FC2154E  imul         rax, qword ptr [rsp+58h]
00007FF69FC21554  mov          rdx, qword ptr [rsp+80h]
00007FF69FC2155C  lea          rdi, [rdx+rax*1]
00007FF69FC21560  add          rdx, rax
00007FF69FC21563  add          rdx, FFFFFFFFFFFFFFFFh
00007FF69FC21567  mov          qword ptr [rsp+50h], rdx
00007FF69FC2156C  mov          qword ptr [rsp+158h], rax
00007FF69FC21574  shl          rax, 4h
00007FF69FC21578  shl          rcx, 6h
00007FF69FC2157C  lea          r14, [rax+rcx*1]
00007FF69FC21580  add          r14, 30h
00007FF69FC21584  mov          qword ptr [rsp+C8h], rbx
00007FF69FC2158C  mov          qword ptr [rsp+40h], rdi
00007FF69FC21591  lea          r15, [rbx+rdi*8]
00007FF69FC21595  add          r15, FFFFFFFFFFFFFFFCh
00007FF69FC21599  xor          r12d, r12d
00007FF69FC2159C  movss        xmm15, dword ptr [__real@3f000000 (00007FF69FC3E544h)]
00007FF69FC215A5  movaps       xmm11, xmmword ptr [__xmm@00000000000000003f80000000000000 (00007FF69FC3E550h)]
00007FF69FC215AD  movaps       xmm13, xmmword ptr [__xmm@0000000000000000bf80000000000000 (00007FF69FC3E560h)]
00007FF69FC215B5  xor          r13d, r13d
00007FF69FC215B8  nop          dword ptr [rax+rax*1], eax
00007FF69FC215C0  test         r13, r13
00007FF69FC215C3  js           static void Fabled_Engine::main+3C0h (00007FF69FC215D0h)
00007FF69FC215C5  xorps        xmm7, xmm7
00007FF69FC215C8  cvtsi2ss     xmm7, r13
00007FF69FC215CD  jmp          static void Fabled_Engine::main+3DBh (00007FF69FC215EBh)
00007FF69FC215CF  nop
00007FF69FC215D0  mov          rax, r13
00007FF69FC215D3  shr          rax, 1h
00007FF69FC215D6  mov          ecx, r13d
00007FF69FC215D9  and          ecx, 1h
00007FF69FC215DC  or           rcx, rax
00007FF69FC215DF  xorps        xmm7, xmm7
00007FF69FC215E2  cvtsi2ss     xmm7, rcx
00007FF69FC215E7  addss        xmm7, xmm7
00007FF69FC215EB  movaps       xmm8, xmm10
00007FF69FC215EF  mulss        xmm8, xmm7
00007FF69FC215F4  addss        xmm7, xmm15
00007FF69FC215F9  movaps       xmm0, xmm8
00007FF69FC215FD  call         sinf (00007FF69FC3BB99h)
00007FF69FC21602  movaps       xmm6, xmm0
00007FF69FC21605  movaps       xmm0, xmm8
00007FF69FC21609  call         cosf (00007FF69FC3BB9Fh)
00007FF69FC2160E  cmp          qword ptr [rsp+38h], r13
00007FF69FC21613  je           static void Fabled_Engine::main+19BAh (00007FF69FC22BCAh)
00007FF69FC21619  mov          rax, qword ptr [rsp+30h]
00007FF69FC2161E  movss        dword ptr [rax+r13*8], xmm0
00007FF69FC21624  movss        dword ptr [rax+r13*8+4h], xmm6
00007FF69FC2162B  cmp          qword ptr [rsp+B0h], r13
00007FF69FC21633  je           static void Fabled_Engine::main+19D0h (00007FF69FC22BE0h)
00007FF69FC21639  mulss        xmm6, xmm15
00007FF69FC2163E  mulss        xmm0, xmm15
00007FF69FC21643  mov          rax, qword ptr [rsp+130h]
00007FF69FC2164B  movss        dword ptr [rax+r13*8], xmm0
00007FF69FC21651  movss        dword ptr [rax+r13*8+4h], xmm6
00007FF69FC21658  mov          rdi, qword ptr [rsp+198h]
00007FF69FC21660  cmp          rdi, r13
00007FF69FC21663  jbe          static void Fabled_Engine::main+19E9h (00007FF69FC22BF9h)
00007FF69FC21669  mov          rax, qword ptr [rsp+188h]
00007FF69FC21671  movaps       xmmword ptr [rax+r12*1], xmm11
00007FF69FC21676  cmp          rbp, r13
00007FF69FC21679  je           static void Fabled_Engine::main+19F8h (00007FF69FC22C08h)
00007FF69FC2167F  mulss        xmm7, xmm14
00007FF69FC21684  movaps       xmm0, xmm12
00007FF69FC21688  subss        xmm0, xmm7
00007FF69FC2168C  mov          rcx, qword ptr [rsp+C8h]
00007FF69FC21694  movss        dword ptr [rcx+r13*8], xmm0
00007FF69FC2169A  mov          dword ptr [rcx+r13*8+4h], 3F800000h
00007FF69FC216A3  mov          rsi, qword ptr [rsp+F8h]
00007FF69FC216AB  cmp          rsi, r13
00007FF69FC216AE  jbe          static void Fabled_Engine::main+1A04h (00007FF69FC22C14h)
00007FF69FC216B4  mov          rbx, qword ptr [rsp+E8h]
00007FF69FC216BC  movaps       xmmword ptr [rbx+r12*1], xmm11
00007FF69FC216C1  mov          rcx, qword ptr [rsp+50h]
00007FF69FC216C6  add          rcx, r13
00007FF69FC216C9  cmp          rdi, rcx
00007FF69FC216CC  jbe          static void Fabled_Engine::main+1A13h (00007FF69FC22C23h)
00007FF69FC216D2  xorps        xmm1, xmm1
00007FF69FC216D5  subps        xmm1, xmmword ptr [rax+r12*1]
00007FF69FC216DA  lea          rdx, [rax+r14*1]
00007FF69FC216DE  movaps       xmmword ptr [r12+rdx*1], xmm1
00007FF69FC216E3  cmp          rbp, rcx
00007FF69FC216E6  jbe          static void Fabled_Engine::main+1A2Ch (00007FF69FC22C3Ch)
00007FF69FC216EC  movss        dword ptr [r15+r13*8-4h], xmm0
00007FF69FC216F3  mov          dword ptr [r15+r13*8], 0h
00007FF69FC216FB  cmp          rsi, rcx
00007FF69FC216FE  jbe          static void Fabled_Engine::main+1A4Ah (00007FF69FC22C5Ah)
00007FF69FC21704  add          r13, 1h
00007FF69FC21708  lea          rcx, [rbx+r14*1]
00007FF69FC2170C  movaps       xmmword ptr [r12+rcx*1], xmm13
00007FF69FC21711  add          r12, 10h
00007FF69FC21715  mov          rcx, qword ptr [rsp+28h]
00007FF69FC2171A  cmp          rcx, r13
00007FF69FC2171D  jne          static void Fabled_Engine::main+3B0h (00007FF69FC215C0h)
00007FF69FC21723  mov          r15, rbp
00007FF69FC21726  mov          rdx, qword ptr [rsp+160h]
00007FF69FC2172E  mov          rbp, qword ptr [rsp+1A8h]
00007FF69FC21736  add          rbp, rdx
00007FF69FC21739  add          rbp, FFFFFFFFFFFFFFFFh
00007FF69FC2173D  imul         rbp, qword ptr [rsp+58h]
00007FF69FC21743  lea          rdx, [rcx*2]
00007FF69FC2174B  add          rdx, rbp
00007FF69FC2174E  mov          qword ptr [rsp+50h], rbp
00007FF69FC21753  lea          r12, [rcx*2+1h]
00007FF69FC2175B  add          r12, rbp
00007FF69FC2175E  mov          r9, r12
00007FF69FC21761  shl          r9, 4h
00007FF69FC21765  lea          rcx, [rbx+r9*1]
00007FF69FC21769  mov          qword ptr [rsp+168h], rcx
00007FF69FC21771  mov          qword ptr [rsp+40h], rdx
00007FF69FC21776  mov          rcx, qword ptr [rsp+C8h]
00007FF69FC2177E  lea          r11, [rcx+rdx*8]
00007FF69FC21782  add          r11, Ch
00007FF69FC21786  add          r9, rax
00007FF69FC21789  mov          r14, qword ptr [rsp+1A0h]
00007FF69FC21791  mov          r13, r14
00007FF69FC21794  shl          r13, 4h
00007FF69FC21798  add          rbx, r13
00007FF69FC2179B  lea          r10, [rcx+r14*8]
00007FF69FC2179F  add          r10, 4h
00007FF69FC217A3  add          r13, rax
00007FF69FC217A6  xor          r8d, r8d
00007FF69FC217A9  movaps       xmm10, xmmword ptr [__xmm@80000000800000008000000080000000 (00007FF69FC3E570h)]
00007FF69FC217B1  movss        xmm0, dword ptr [__real@3f000000 (00007FF69FC3E544h)]
00007FF69FC217B9  movss        xmm1, dword ptr [__real@bf000000 (00007FF69FC3E580h)]
00007FF69FC217C1  xor          ebp, ebp
00007FF69FC217C3  nop          word ptr cs:[rax+rax*1], ax
00007FF69FC217CD  nop          dword ptr [rax], eax
00007FF69FC217D0  test         rbp, rbp
00007FF69FC217D3  js           static void Fabled_Engine::main+5D0h (00007FF69FC217E0h)
00007FF69FC217D5  xorps        xmm3, xmm3
00007FF69FC217D8  cvtsi2ss     xmm3, rbp
00007FF69FC217DD  jmp          static void Fabled_Engine::main+5EAh (00007FF69FC217FAh)
00007FF69FC217DF  nop
00007FF69FC217E0  mov          rax, rbp
00007FF69FC217E3  shr          rax, 1h
00007FF69FC217E6  mov          ecx, ebp
00007FF69FC217E8  and          ecx, 1h
00007FF69FC217EB  or           rcx, rax
00007FF69FC217EE  xorps        xmm3, xmm3
00007FF69FC217F1  cvtsi2ss     xmm3, rcx
00007FF69FC217F6  addss        xmm3, xmm3
00007FF69FC217FA  mulss        xmm3, xmm14
00007FF69FC217FF  movaps       xmm2, xmm12
00007FF69FC21803  subss        xmm2, xmm3
00007FF69FC21807  mov          rax, qword ptr [rsp+88h]
00007FF69FC2180F  movss        dword ptr [rax+r8*1], xmm2
00007FF69FC21815  mov          rax, rbp
00007FF69FC21818  shr          rax, 20h
00007FF69FC2181C  je           static void Fabled_Engine::main+630h (00007FF69FC21840h)
00007FF69FC2181E  mov          rax, rbp
00007FF69FC21821  xor          edx, edx
00007FF69FC21823  div          qword ptr [rsp+28h]
00007FF69FC21828  cmp          qword ptr [rsp+38h], rdx
00007FF69FC2182D  ja           static void Fabled_Engine::main+643h (00007FF69FC21853h)
00007FF69FC2182F  jmp          static void Fabled_Engine::main+1A63h (00007FF69FC22C73h)
00007FF69FC21834  nop          word ptr cs:[rax+rax*1], ax
00007FF69FC2183E  nop
00007FF69FC21840  mov          eax, ebp
00007FF69FC21842  xor          edx, edx
00007FF69FC21844  div          dword ptr [rsp+28h]
00007FF69FC21848  cmp          qword ptr [rsp+38h], rdx
00007FF69FC2184D  jbe          static void Fabled_Engine::main+1A63h (00007FF69FC22C73h)
00007FF69FC21853  cmp          qword ptr [rsp+B0h], rdx
00007FF69FC2185B  jbe          static void Fabled_Engine::main+1A79h (00007FF69FC22C89h)
00007FF69FC21861  lea          rcx, [r14+rbp*1]
00007FF69FC21865  cmp          rdi, rcx
00007FF69FC21868  jbe          static void Fabled_Engine::main+1A92h (00007FF69FC22CA2h)
00007FF69FC2186E  mov          rax, qword ptr [rsp+30h]
00007FF69FC21873  movss        xmm6, dword ptr [rax+rdx*8]
00007FF69FC21878  movss        xmm5, dword ptr [rax+rdx*8+4h]
00007FF69FC2187E  mov          rax, qword ptr [rsp+130h]
00007FF69FC21886  movss        xmm4, dword ptr [rax+rdx*8]
00007FF69FC2188B  movss        xmm3, dword ptr [rax+rdx*8+4h]
00007FF69FC21891  xorps        xmm3, xmm10
00007FF69FC21895  movaps       xmm7, xmm0
00007FF69FC21898  movlhps      xmm7, xmm4
00007FF69FC2189B  shufps       xmm7, xmm3, 2h
00007FF69FC2189F  movaps       xmmword ptr [r13+r8*4], xmm7
00007FF69FC218A5  cmp          r15, rcx
00007FF69FC218A8  jbe          static void Fabled_Engine::main+1A9Bh (00007FF69FC22CABh)
00007FF69FC218AE  movss        dword ptr [r10+r8*2-4h], xmm2
00007FF69FC218B5  mov          dword ptr [r10+r8*2], 3F400000h
00007FF69FC218BD  cmp          rsi, rcx
00007FF69FC218C0  jbe          static void Fabled_Engine::main+1AA4h (00007FF69FC22CB4h)
00007FF69FC218C6  xorps        xmm5, xmm10
00007FF69FC218CA  xorps        xmm7, xmm7
00007FF69FC218CD  movss        xmm7, xmm6
00007FF69FC218D1  shufps       xmm7, xmm5, 4h
00007FF69FC218D5  movaps       xmmword ptr [rbx+r8*4], xmm7
00007FF69FC218DA  lea          rax, [r12+rbp*1]
00007FF69FC218DE  cmp          rdi, rax
00007FF69FC218E1  jbe          static void Fabled_Engine::main+1AADh (00007FF69FC22CBDh)
00007FF69FC218E7  movaps       xmm5, xmm1
00007FF69FC218EA  movlhps      xmm5, xmm4
00007FF69FC218ED  shufps       xmm5, xmm3, 2h
00007FF69FC218F1  movaps       xmmword ptr [r9+r8*4], xmm5
00007FF69FC218F6  cmp          r15, rax
00007FF69FC218F9  jbe          static void Fabled_Engine::main+1ACBh (00007FF69FC22CDBh)
00007FF69FC218FF  movss        dword ptr [r11+r8*2-4h], xmm2
00007FF69FC21906  mov          dword ptr [r11+r8*2], 3E800000h
00007FF69FC2190E  cmp          rsi, rax
00007FF69FC21911  jbe          static void Fabled_Engine::main+1AE9h (00007FF69FC22CF9h)
00007FF69FC21917  add          rbp, 1h
00007FF69FC2191B  movaps       xmm2, xmmword ptr [rbx+r8*4]
00007FF69FC21920  mov          rax, qword ptr [rsp+168h]
00007FF69FC21928  movaps       xmmword ptr [rax+r8*4], xmm2
00007FF69FC2192D  add          r8, 4h
00007FF69FC21931  cmp          qword ptr [rsp+80h], r8
00007FF69FC21939  jne          static void Fabled_Engine::main+5C0h (00007FF69FC217D0h)
00007FF69FC2193F  cmp          qword ptr [rsp+78h], 0h
00007FF69FC21945  mov          rbx, qword ptr [rsp+28h]
00007FF69FC2194A  je           static void Fabled_Engine::main+ABAh (00007FF69FC21CCAh)
00007FF69FC21950  mov          rax, qword ptr [rsp+60h]
00007FF69FC21955  mov          qword ptr [rsp+C8h], rax
00007FF69FC2195D  mov          rax, qword ptr [rsp+70h]
00007FF69FC21962  mov          qword ptr [rsp+1F0h], rax
00007FF69FC2196A  mov          rax, qword ptr [rsp+D0h]
00007FF69FC21972  mov          rcx, qword ptr [rsp+E0h]
00007FF69FC2197A  mov          qword ptr [rsp+1F8h], rcx
00007FF69FC21982  lea          rcx, [rbx+rbx*2]
00007FF69FC21986  mov          rbp, qword ptr [rsp+50h]
00007FF69FC2198B  lea          rdx, [rcx+rbp*1]
00007FF69FC2198F  lea          rdi, [rcx+rbp*1]
00007FF69FC21993  add          rdi, 2h
00007FF69FC21997  mov          qword ptr [rsp+30h], rdi
00007FF69FC2199C  shl          rbp, 4h
00007FF69FC219A0  shl          rcx, 4h
00007FF69FC219A4  lea          r12, [rcx+rbp*1]
00007FF69FC219A8  add          r12, 20h
00007FF69FC219AC  mov          r13, rbx
00007FF69FC219AF  shl          r13, 4h
00007FF69FC219B3  lea          rcx, [r13+10h]
00007FF69FC219B7  mov          qword ptr [rsp+1E8h], rcx
00007FF69FC219BF  lea          r14, [rax+rdx*8]
00007FF69FC219C3  add          r14, 14h
00007FF69FC219C7  lea          rcx, [rbx*8+8h]
00007FF69FC219CF  mov          qword ptr [rsp+1E0h], rcx
00007FF69FC219D7  lea          r15, [rax+rbx*8]
00007FF69FC219DB  add          r15, 4h
00007FF69FC219DF  xor          esi, esi
00007FF69FC219E1  mov          qword ptr [rsp+38h], rbx
00007FF69FC219E6  nop          word ptr cs:[rax+rax*1], ax
00007FF69FC219F0  test         rsi, rsi
00007FF69FC219F3  js           static void Fabled_Engine::main+7F0h (00007FF69FC21A00h)
00007FF69FC219F5  xorps        xmm6, xmm6
00007FF69FC219F8  cvtsi2ss     xmm6, rsi
00007FF69FC219FD  jmp          static void Fabled_Engine::main+80Ah (00007FF69FC21A1Ah)
00007FF69FC219FF  nop
00007FF69FC21A00  mov          rax, rsi
00007FF69FC21A03  shr          rax, 1h
00007FF69FC21A06  mov          ecx, esi
00007FF69FC21A08  and          ecx, 1h
00007FF69FC21A0B  or           rcx, rax
00007FF69FC21A0E  xorps        xmm6, xmm6
00007FF69FC21A11  cvtsi2ss     xmm6, rcx
00007FF69FC21A16  addss        xmm6, xmm6
00007FF69FC21A1A  addss        xmm6, xmm12
00007FF69FC21A1F  movss        xmm8, dword ptr [rsp+184h]
00007FF69FC21A29  mulss        xmm8, xmm6
00007FF69FC21A2E  movaps       xmm0, xmm8
00007FF69FC21A32  call         sinf (00007FF69FC3BB99h)
00007FF69FC21A37  movaps       xmm7, xmm0
00007FF69FC21A3A  movaps       xmm0, xmm8
00007FF69FC21A3E  call         cosf (00007FF69FC3BB9Fh)
00007FF69FC21A43  mulss        xmm6, dword ptr [rsp+230h]
00007FF69FC21A4C  movaps       xmm1, xmm12
00007FF69FC21A50  subss        xmm1, xmm6
00007FF69FC21A54  mulss        xmm6, dword ptr [__real@3f400000 (00007FF69FC3E584h)]
00007FF69FC21A5C  movaps       xmm2, xmm1
00007FF69FC21A5F  mulss        xmm2, dword ptr [__real@3e800000 (00007FF69FC3E588h)]
00007FF69FC21A67  cmp          byte ptr [rsp+48h], 0h
00007FF69FC21A6C  je           static void Fabled_Engine::main+1CDCh (00007FF69FC22EECh)
00007FF69FC21A72  add          rsi, 1h
00007FF69FC21A76  mov          qword ptr [rsp+168h], rsi
00007FF69FC21A7E  movaps       xmm3, xmm0
00007FF69FC21A81  xorps        xmm3, xmm10
00007FF69FC21A85  movaps       xmmword ptr [rsp+130h], xmm3
00007FF69FC21A8D  movaps       xmm3, xmm10
00007FF69FC21A91  movaps       xmm10, xmm7
00007FF69FC21A95  mulss        xmm10, xmm15
00007FF69FC21A9A  movaps       xmm13, xmm0
00007FF69FC21A9E  mulss        xmm13, xmm15
00007FF69FC21AA3  movaps       xmm4, xmm13
00007FF69FC21AA7  addss        xmm4, xmm15
00007FF69FC21AAC  movaps       xmmword ptr [rsp+B0h], xmm4
00007FF69FC21AB4  movss        xmm14, dword ptr [__real@bf000000 (00007FF69FC3E580h)]
00007FF69FC21ABD  subss        xmm14, xmm10
00007FF69FC21AC2  addss        xmm1, xmm6
00007FF69FC21AC6  movaps       xmm4, xmm10
00007FF69FC21ACA  xorps        xmm4, xmm3
00007FF69FC21ACD  movaps       xmm15, xmm7
00007FF69FC21AD1  xorps        xmm15, xmm3
00007FF69FC21AD5  movaps       xmm5, xmm13
00007FF69FC21AD9  xorps        xmm5, xmm3
00007FF69FC21ADC  xor          edi, edi
00007FF69FC21ADE  xor          esi, esi
00007FF69FC21AE0  mov          r8, qword ptr [rsp+28h]
00007FF69FC21AE5  mov          r9, qword ptr [rsp+80h]
00007FF69FC21AED  mov          r10, qword ptr [rsp+1F8h]
00007FF69FC21AF5  mov          r11, qword ptr [rsp+1F0h]
00007FF69FC21AFD  nop          dword ptr [rax], eax
00007FF69FC21B00  mov          rax, qword ptr [rsp+88h]
00007FF69FC21B08  movss        xmm12, dword ptr [rax+rdi*1]
00007FF69FC21B0E  mov          rax, rsi
00007FF69FC21B11  shr          rax, 20h
00007FF69FC21B15  je           static void Fabled_Engine::main+920h (00007FF69FC21B30h)
00007FF69FC21B17  mov          rax, rsi
00007FF69FC21B1A  xor          edx, edx
00007FF69FC21B1C  div          r8
00007FF69FC21B1F  mov          rax, rdx
00007FF69FC21B22  cmp          r11, rax
00007FF69FC21B25  ja           static void Fabled_Engine::main+932h (00007FF69FC21B42h)
00007FF69FC21B27  jmp          static void Fabled_Engine::main+12BAh (00007FF69FC224CAh)
00007FF69FC21B2C  nop          dword ptr [rax], eax
00007FF69FC21B30  mov          eax, esi
00007FF69FC21B32  xor          edx, edx
00007FF69FC21B34  div          r8d
00007FF69FC21B37  mov          eax, edx
00007FF69FC21B39  cmp          r11, rax
00007FF69FC21B3C  jbe          static void Fabled_Engine::main+12BAh (00007FF69FC224CAh)
00007FF69FC21B42  mov          rcx, qword ptr [rsp+38h]
00007FF69FC21B47  add          rcx, rsi
00007FF69FC21B4A  mov          rdx, qword ptr [rsp+198h]
00007FF69FC21B52  cmp          rdx, rcx
00007FF69FC21B55  jbe          static void Fabled_Engine::main+12CEh (00007FF69FC224DEh)
00007FF69FC21B5B  mov          rbx, qword ptr [rsp+C8h]
00007FF69FC21B63  movss        xmm8, dword ptr [rbx+rax*8]
00007FF69FC21B69  movss        xmm9, dword ptr [rbx+rax*8+4h]
00007FF69FC21B70  movaps       xmm6, xmm9
00007FF69FC21B74  mulss        xmm6, xmm4
00007FF69FC21B78  mov          rbx, qword ptr [rsp+188h]
00007FF69FC21B80  movaps       xmm11, xmm10
00007FF69FC21B84  mulss        xmm11, xmm8
00007FF69FC21B89  unpcklps     xmm11, xmmword ptr [rsp+B0h]
00007FF69FC21B92  shufps       xmm11, xmm6, 4h
00007FF69FC21B97  lea          rax, [rbx+r13*1]
00007FF69FC21B9B  movaps       xmmword ptr [rax+rdi*4], xmm11
00007FF69FC21BA0  cmp          r10, rcx
00007FF69FC21BA3  jbe          static void Fabled_Engine::main+12DCh (00007FF69FC224ECh)
00007FF69FC21BA9  movss        dword ptr [r15+rdi*2-4h], xmm12
00007FF69FC21BB0  movss        dword ptr [r15+rdi*2], xmm1
00007FF69FC21BB6  mov          rax, qword ptr [rsp+F8h]
00007FF69FC21BBE  cmp          rax, rcx
00007FF69FC21BC1  jbe          static void Fabled_Engine::main+12EDh (00007FF69FC224FDh)
00007FF69FC21BC7  movaps       xmm6, xmm9
00007FF69FC21BCB  mulss        xmm6, xmm15
00007FF69FC21BD0  mov          rbp, qword ptr [rsp+E8h]
00007FF69FC21BD8  movaps       xmm3, xmm7
00007FF69FC21BDB  mulss        xmm3, xmm8
00007FF69FC21BE0  unpcklps     xmm3, xmm0
00007FF69FC21BE3  shufps       xmm3, xmm6, 4h
00007FF69FC21BE7  lea          rcx, [rbp+r13*1]
00007FF69FC21BEC  movaps       xmmword ptr [rcx+rdi*4], xmm3
00007FF69FC21BF0  mov          rcx, qword ptr [rsp+30h]
00007FF69FC21BF5  add          rcx, rsi
00007FF69FC21BF8  cmp          rdx, rcx
00007FF69FC21BFB  jbe          static void Fabled_Engine::main+12FEh (00007FF69FC2250Eh)
00007FF69FC21C01  movaps       xmm3, xmm9
00007FF69FC21C05  mulss        xmm3, xmm5
00007FF69FC21C09  movaps       xmm6, xmm13
00007FF69FC21C0D  mulss        xmm6, xmm8
00007FF69FC21C12  unpcklps     xmm6, xmm14
00007FF69FC21C16  shufps       xmm6, xmm3, 4h
00007FF69FC21C1A  add          rbx, r12
00007FF69FC21C1D  movaps       xmmword ptr [rbx+rdi*4], xmm6
00007FF69FC21C21  cmp          r10, rcx
00007FF69FC21C24  jbe          static void Fabled_Engine::main+130Ch (00007FF69FC2251Ch)
00007FF69FC21C2A  movss        dword ptr [r14+rdi*2-4h], xmm12
00007FF69FC21C31  movss        dword ptr [r14+rdi*2], xmm2
00007FF69FC21C37  cmp          rax, rcx
00007FF69FC21C3A  jbe          static void Fabled_Engine::main+131Dh (00007FF69FC2252Dh)
00007FF69FC21C40  add          rsi, 1h
00007FF69FC21C44  mulss        xmm9, dword ptr [rsp+130h]
00007FF69FC21C4E  mulss        xmm8, xmm0
00007FF69FC21C53  unpcklps     xmm8, xmm15
00007FF69FC21C57  shufps       xmm8, xmm9, 4h
00007FF69FC21C5C  add          rbp, r12
00007FF69FC21C5F  movaps       xmmword ptr [rbp+rdi*4], xmm8
00007FF69FC21C65  add          rdi, 4h
00007FF69FC21C69  cmp          r9, rdi
00007FF69FC21C6C  jne          static void Fabled_Engine::main+8F0h (00007FF69FC21B00h)
00007FF69FC21C72  mov          rax, qword ptr [rsp+58h]
00007FF69FC21C77  add          qword ptr [rsp+30h], rax
00007FF69FC21C7C  mov          rcx, qword ptr [rsp+1E8h]
00007FF69FC21C84  add          r12, rcx
00007FF69FC21C87  mov          rdx, qword ptr [rsp+1E0h]
00007FF69FC21C8F  add          r14, rdx
00007FF69FC21C92  add          qword ptr [rsp+38h], rax
00007FF69FC21C97  add          r13, rcx
00007FF69FC21C9A  add          r15, rdx
00007FF69FC21C9D  mov          rsi, qword ptr [rsp+168h]
00007FF69FC21CA5  cmp          rsi, qword ptr [rsp+78h]
00007FF69FC21CAA  movss        xmm12, dword ptr [__real@3f800000 (00007FF69FC3E534h)]
00007FF69FC21CB3  movss        xmm15, dword ptr [__real@3f000000 (00007FF69FC3E544h)]
00007FF69FC21CBC  movaps       xmm10, xmmword ptr [__xmm@80000000800000008000000080000000 (00007FF69FC3E570h)]
00007FF69FC21CC4  jne          static void Fabled_Engine::main+7E0h (00007FF69FC219F0h)
00007FF69FC21CCA  movss        xmm0, dword ptr [__real@5f000000 (00007FF69FC3E58Ch)]
00007FF69FC21CD2  movaps       xmm2, xmmword ptr [rsp+220h]
00007FF69FC21CDA  movaps       xmm1, xmm2
00007FF69FC21CDD  subss        xmm1, xmm0
00007FF69FC21CE1  cvttss2si    rax, xmm1
00007FF69FC21CE6  mov          rcx, 8000000000000000h
00007FF69FC21CF0  xor          rcx, rax
00007FF69FC21CF3  cvttss2si    rax, xmm2
00007FF69FC21CF8  ucomiss      xmm2, xmm0
00007FF69FC21CFB  cmovae       rax, rcx
00007FF69FC21CFF  xor          ecx, ecx
00007FF69FC21D01  xorps        xmm0, xmm0
00007FF69FC21D04  ucomiss      xmm2, xmm0
00007FF69FC21D07  cmovae       rcx, rax
00007FF69FC21D0B  ucomiss      xmm2, dword ptr [__real@5f7fffff (00007FF69FC3E590h)]
00007FF69FC21D12  mov          r8, FFFFFFFFFFFFFFFFh
00007FF69FC21D19  cmovbe       r8, rcx
00007FF69FC21D1D  mov          rsi, qword ptr [rsp+C0h]
00007FF69FC21D25  imul         r8, rsi
00007FF69FC21D29  cmp          r8, 2h
00007FF69FC21D2D  jb           static void Fabled_Engine::main+D13h (00007FF69FC21F23h)
00007FF69FC21D33  cmp          byte ptr [rsp+48h], 0h
00007FF69FC21D38  je           static void Fabled_Engine::main+1D7Fh (00007FF69FC22F8Fh)
00007FF69FC21D3E  xorps        xmm1, xmm1
00007FF69FC21D41  cvtsi2ss     xmm1, esi
00007FF69FC21D45  movaps       xmm0, xmm12
00007FF69FC21D49  divss        xmm0, xmm1
00007FF69FC21D4D  mov          rdi, qword ptr [rsp+60h]
00007FF69FC21D52  mov          r10, qword ptr [rsp+70h]
00007FF69FC21D57  mov          rbp, qword ptr [rsp+90h]
00007FF69FC21D5F  mov          r12, qword ptr [rsp+A0h]
00007FF69FC21D67  mov          r11, qword ptr [rsp+D0h]
00007FF69FC21D6F  mov          r9, qword ptr [rsp+E0h]
00007FF69FC21D77  add          r11, 4h
00007FF69FC21D7B  mov          r14d, 1h
00007FF69FC21D81  movss        xmm8, dword ptr [__real@3f400000 (00007FF69FC3E584h)]
00007FF69FC21D8A  movss        xmm9, dword ptr [__real@3e800000 (00007FF69FC3E588h)]
00007FF69FC21D93  nop          word ptr cs:[rax+rax*1], ax
00007FF69FC21D9D  nop          dword ptr [rax], eax
00007FF69FC21DA0  test         r14, r14
00007FF69FC21DA3  js           static void Fabled_Engine::main+BA0h (00007FF69FC21DB0h)
00007FF69FC21DA5  xorps        xmm3, xmm3
00007FF69FC21DA8  cvtsi2ss     xmm3, r14
00007FF69FC21DAD  jmp          static void Fabled_Engine::main+BBBh (00007FF69FC21DCBh)
00007FF69FC21DAF  nop
00007FF69FC21DB0  mov          rax, r14
00007FF69FC21DB3  shr          rax, 1h
00007FF69FC21DB6  mov          ecx, r14d
00007FF69FC21DB9  and          ecx, 1h
00007FF69FC21DBC  or           rcx, rax
00007FF69FC21DBF  xorps        xmm3, xmm3
00007FF69FC21DC2  cvtsi2ss     xmm3, rcx
00007FF69FC21DC7  addss        xmm3, xmm3
00007FF69FC21DCB  mulss        xmm3, xmm0
00007FF69FC21DCF  add          r14, 1h
00007FF69FC21DD3  movaps       xmm5, xmm12
00007FF69FC21DD7  subss        xmm5, xmm3
00007FF69FC21DDB  mulss        xmm5, xmm8
00007FF69FC21DE0  movaps       xmm4, xmm15
00007FF69FC21DE4  subss        xmm4, xmm3
00007FF69FC21DE8  mulss        xmm3, xmm9
00007FF69FC21DED  mov          r13, r9
00007FF69FC21DF0  mov          rax, qword ptr [rsp+170h]
00007FF69FC21DF8  sub          r13, rax
00007FF69FC21DFB  mov          ecx, 0h
00007FF69FC21E00  cmovb        r13, rcx
00007FF69FC21E04  addss        xmm3, xmm5
00007FF69FC21E08  mov          rbx, rax
00007FF69FC21E0B  shl          rbx, 4h
00007FF69FC21E0F  lea          r15, [r11+rax*8]
00007FF69FC21E13  xor          esi, esi
00007FF69FC21E15  nop          word ptr cs:[rax+rax*1], ax
00007FF69FC21E1F  nop
00007FF69FC21E20  mov          rax, qword ptr [rsp+88h]
00007FF69FC21E28  movss        xmm5, dword ptr [rax+rsi*4]
00007FF69FC21E2D  mov          rax, rsi
00007FF69FC21E30  shr          rax, 20h
00007FF69FC21E34  je           static void Fabled_Engine::main+C40h (00007FF69FC21E50h)
00007FF69FC21E36  mov          rax, rsi
00007FF69FC21E39  xor          edx, edx
00007FF69FC21E3B  div          qword ptr [rsp+28h]
00007FF69FC21E40  cmp          r10, rdx
00007FF69FC21E43  ja           static void Fabled_Engine::main+C51h (00007FF69FC21E61h)
00007FF69FC21E45  jmp          static void Fabled_Engine::main+132Eh (00007FF69FC2253Eh)
00007FF69FC21E4A  nop          word ptr [rax+rax*1], ax
00007FF69FC21E50  mov          eax, esi
00007FF69FC21E52  xor          edx, edx
00007FF69FC21E54  div          dword ptr [rsp+28h]
00007FF69FC21E58  cmp          r10, rdx
00007FF69FC21E5B  jbe          static void Fabled_Engine::main+132Eh (00007FF69FC2253Eh)
00007FF69FC21E61  cmp          r12, rdx
00007FF69FC21E64  jbe          static void Fabled_Engine::main+1342h (00007FF69FC22552h)
00007FF69FC21E6A  mov          rax, qword ptr [rsp+170h]
00007FF69FC21E72  lea          rcx, [rax+rsi*1]
00007FF69FC21E76  mov          rax, qword ptr [rsp+198h]
00007FF69FC21E7E  cmp          rax, rcx
00007FF69FC21E81  jbe          static void Fabled_Engine::main+1356h (00007FF69FC22566h)
00007FF69FC21E87  movss        xmm7, dword ptr [rdi+rdx*8]
00007FF69FC21E8C  movss        xmm6, dword ptr [rdi+rdx*8+4h]
00007FF69FC21E92  movss        xmm1, dword ptr [rbp+rdx*8+4h]
00007FF69FC21E98  xorps        xmm1, xmm10
00007FF69FC21E9C  mov          rax, qword ptr [rsp+188h]
00007FF69FC21EA4  movss        xmm2, dword ptr [rbp+rdx*8]
00007FF69FC21EAA  unpcklps     xmm2, xmm4
00007FF69FC21EAD  shufps       xmm2, xmm1, 4h
00007FF69FC21EB1  movaps       xmmword ptr [rax+rbx*1], xmm2
00007FF69FC21EB5  cmp          r13, rsi
00007FF69FC21EB8  je           static void Fabled_Engine::main+1367h (00007FF69FC22577h)
00007FF69FC21EBE  movss        dword ptr [r15+rsi*8-4h], xmm5
00007FF69FC21EC5  movss        dword ptr [r15+rsi*8], xmm3
00007FF69FC21ECB  mov          rdx, qword ptr [rsp+F8h]
00007FF69FC21ED3  cmp          rdx, rcx
00007FF69FC21ED6  jbe          static void Fabled_Engine::main+1378h (00007FF69FC22588h)
00007FF69FC21EDC  add          rsi, 1h
00007FF69FC21EE0  xorps        xmm6, xmm10
00007FF69FC21EE4  mov          rax, qword ptr [rsp+E8h]
00007FF69FC21EEC  xorps        xmm1, xmm1
00007FF69FC21EEF  movss        xmm1, xmm7
00007FF69FC21EF3  shufps       xmm1, xmm6, 4h
00007FF69FC21EF7  movaps       xmmword ptr [rax+rbx*1], xmm1
00007FF69FC21EFB  add          rbx, 10h
00007FF69FC21EFF  cmp          qword ptr [rsp+58h], rsi
00007FF69FC21F04  jne          static void Fabled_Engine::main+C10h (00007FF69FC21E20h)
00007FF69FC21F0A  add          qword ptr [rsp+170h], rsi
00007FF69FC21F12  cmp          r14, r8
00007FF69FC21F15  mov          rsi, qword ptr [rsp+C0h]
00007FF69FC21F1D  jne          static void Fabled_Engine::main+B90h (00007FF69FC21DA0h)
00007FF69FC21F23  mov          rax, qword ptr [rsp+28h]
00007FF69FC21F28  lea          r12, [rax+rax*2]
00007FF69FC21F2C  add          rax, rax
00007FF69FC21F2F  mov          qword ptr [rsp+38h], rax
00007FF69FC21F34  lea          rax, [rax+rax*2]
00007FF69FC21F38  mov          r14, qword ptr [rsp+78h]
00007FF69FC21F3D  imul         r14, rax
00007FF69FC21F41  lea          r13, [r14+r12*1]
00007FF69FC21F45  mov          qword ptr [rsp+B0h], rax
00007FF69FC21F4D  imul         rsi, rax
00007FF69FC21F51  add          rsi, r13
00007FF69FC21F54  add          r14, rsi
00007FF69FC21F57  add          r14, r12
00007FF69FC21F5A  mov          ecx, 8h
00007FF69FC21F5F  mov          rax, r14
00007FF69FC21F62  mul          rcx
00007FF69FC21F65  jo           static void Fabled_Engine::main+1DE6h (00007FF69FC22FF6h)
00007FF69FC21F6B  mov          rdi, rax
00007FF69FC21F6E  test         rax, rax
00007FF69FC21F71  je           static void Fabled_Engine::main+DA0h (00007FF69FC21FB0h)
00007FF69FC21F73  mov          rcx, qword ptr [00007FF69FC451C8h]
00007FF69FC21F7A  test         rcx, rcx
00007FF69FC21F7D  jne          static void Fabled_Engine::main+D87h (00007FF69FC21F97h)
00007FF69FC21F7F  call         GetProcessHeap (00007FF69FC3AA6Ch)
00007FF69FC21F84  test         rax, rax
00007FF69FC21F87  je           static void Fabled_Engine::main+1DEDh (00007FF69FC22FFDh)
00007FF69FC21F8D  mov          rcx, rax
00007FF69FC21F90  mov          qword ptr [00007FF69FC451C8h], rax
00007FF69FC21F97  mov          edx, 8h
00007FF69FC21F9C  mov          r8, rdi
00007FF69FC21F9F  call         HeapAlloc (00007FF69FC3AA72h)
00007FF69FC21FA4  mov          rcx, rax
00007FF69FC21FA7  test         rax, rax
00007FF69FC21FAA  je           static void Fabled_Engine::main+1DEDh (00007FF69FC22FFDh)
00007FF69FC21FB0  mov          qword ptr [rsp+30h], rcx
00007FF69FC21FB5  mov          qword ptr [rsp+130h], rdi
00007FF69FC21FBD  mov          qword ptr [rsp+C0h], rsi
00007FF69FC21FC5  cmp          byte ptr [rsp+48h], 0h
00007FF69FC21FCA  je           static void Fabled_Engine::main+F1Ch (00007FF69FC2212Ch)
00007FF69FC21FD0  mov          rax, qword ptr [rsp+1A8h]
00007FF69FC21FD8  shl          rax, 2h
00007FF69FC21FDC  lea          rsi, [rax+rax*2]
00007FF69FC21FE0  mov          rax, qword ptr [rsp+160h]
00007FF69FC21FE8  lea          rcx, [rax+rax*1]
00007FF69FC21FEC  lea          rcx, [rcx+rcx*2]
00007FF69FC21FF0  lea          rdx, [rsi+rcx*1]
00007FF69FC21FF4  mov          rax, qword ptr [rsp+28h]
00007FF69FC21FF9  imul         rdx, rax
00007FF69FC21FFD  mov          edi, AAAAAAABh
00007FF69FC22002  mov          rbx, rdx
00007FF69FC22005  imul         rbx, rdi
00007FF69FC22009  mov          rbp, 155555556h
00007FF69FC22013  add          rbp, rbx
00007FF69FC22016  shr          rbp, 21h
00007FF69FC2201A  mov          r10, rdx
00007FF69FC2201D  or           r10, 1h
00007FF69FC22021  imul         r10, rdi
00007FF69FC22025  shr          r10, 21h
00007FF69FC22029  test         rdx, rdx
00007FF69FC2202C  cmove        r10, rdx
00007FF69FC22030  cmp          rdx, 2h
00007FF69FC22034  mov          ebx, 2h
00007FF69FC22039  cmova        rbx, rdx
00007FF69FC2203D  imul         rbx, rdi
00007FF69FC22041  shr          rbx, 21h
00007FF69FC22045  lea          rdi, [rsi+rcx*1]
00007FF69FC22049  add          rdi, FFFFFFFFFFFFFFFDh
00007FF69FC2204D  imul         rdi, rax
00007FF69FC22051  mov          rcx, qword ptr [rsp+30h]
00007FF69FC22056  lea          rsi, [rcx+rdi*8]
00007FF69FC2205A  add          rsi, 10h
00007FF69FC2205E  lea          r8, [rbx+rbx*2]
00007FF69FC22062  lea          r9, [rbp+rbp*2]
00007FF69FC22067  mov          rcx, qword ptr [rsp+158h]
00007FF69FC2206F  lea          rdx, [rcx+r12*1]
00007FF69FC22073  lea          r11, [rcx+rax*4]
00007FF69FC22077  add          r11, 3h
00007FF69FC2207B  xor          ecx, ecx
00007FF69FC2207D  xor          r15d, r15d
00007FF69FC22080  mov          rbp, qword ptr [rsp+30h]
00007FF69FC22085  nop          word ptr cs:[rax+rax*1], ax
00007FF69FC2208F  nop
00007FF69FC22090  cmp          r9, rcx
00007FF69FC22093  je           static void Fabled_Engine::main+1C65h (00007FF69FC22E75h)
00007FF69FC22099  mov          qword ptr [rbp+rcx*8], r15
00007FF69FC2209E  cmp          r10, r15
00007FF69FC220A1  je           static void Fabled_Engine::main+1C76h (00007FF69FC22E86h)
00007FF69FC220A7  mov          rbx, qword ptr [rsp+28h]
00007FF69FC220AC  lea          rax, [rbx+r15*1]
00007FF69FC220B0  mov          qword ptr [rbp+rcx*8+8h], rax
00007FF69FC220B5  cmp          r8, rcx
00007FF69FC220B8  je           static void Fabled_Engine::main+1C8Bh (00007FF69FC22E9Bh)
00007FF69FC220BE  lea          rax, [rdi+rcx*1]
00007FF69FC220C2  add          rbx, r15
00007FF69FC220C5  add          rbx, 1h
00007FF69FC220C9  mov          qword ptr [rbp+rcx*8+10h], rbx
00007FF69FC220CE  cmp          r14, rax
00007FF69FC220D1  jbe          static void Fabled_Engine::main+1CA0h (00007FF69FC22EB0h)
00007FF69FC220D7  lea          rax, [r11+r15*1]
00007FF69FC220DB  mov          qword ptr [rsi+rcx*8-10h], rax
00007FF69FC220E0  lea          rax, [rdi+rcx*1]
00007FF69FC220E4  add          rax, 1h
00007FF69FC220E8  cmp          r14, rax
00007FF69FC220EB  jbe          static void Fabled_Engine::main+1CB4h (00007FF69FC22EC4h)
00007FF69FC220F1  lea          rax, [rdx+r15*1]
00007FF69FC220F5  add          rax, 3h
00007FF69FC220F9  mov          qword ptr [rsi+rcx*8-8h], rax
00007FF69FC220FE  lea          rax, [rdi+rcx*1]
00007FF69FC22102  add          rax, 2h
00007FF69FC22106  cmp          r14, rax
00007FF69FC22109  jbe          static void Fabled_Engine::main+1CC8h (00007FF69FC22ED8h)
00007FF69FC2210F  lea          rax, [rdx+r15*1]
00007FF69FC22113  add          rax, 2h
00007FF69FC22117  mov          qword ptr [rsi+rcx*8], rax
00007FF69FC2211B  add          r15, 1h
00007FF69FC2211F  add          rcx, 3h
00007FF69FC22123  cmp          r12, rcx
00007FF69FC22126  jne          static void Fabled_Engine::main+E80h (00007FF69FC22090h)
00007FF69FC2212C  cmp          qword ptr [rsp+78h], 0h
00007FF69FC22132  mov          r15, qword ptr [rsp+C0h]
00007FF69FC2213A  je           static void Fabled_Engine::main+10F6h (00007FF69FC22306h)
00007FF69FC22140  add          qword ptr [rsp+50h], r12
00007FF69FC22145  mov          rax, qword ptr [rsp+30h]
00007FF69FC2214A  lea          r8, [rax+28h]
00007FF69FC2214E  xor          r9d, r9d
00007FF69FC22151  xor          r10d, r10d
00007FF69FC22154  mov          r11, qword ptr [rsp+28h]
00007FF69FC22159  jmp          static void Fabled_Engine::main+F64h (00007FF69FC22174h)
00007FF69FC2215B  nop          dword ptr [rax+rax*1], eax
00007FF69FC22160  add          r10, 1h
00007FF69FC22164  add          r9, qword ptr [rsp+58h]
00007FF69FC22169  cmp          r10, qword ptr [rsp+78h]
00007FF69FC2216E  je           static void Fabled_Engine::main+10F6h (00007FF69FC22306h)
00007FF69FC22174  cmp          byte ptr [rsp+48h], 0h
00007FF69FC22179  je           static void Fabled_Engine::main+F50h (00007FF69FC22160h)
00007FF69FC2217B  lea          rax, [r8+r15*8]
00007FF69FC2217F  lea          rsi, [r8+r12*8]
00007FF69FC22183  mov          rdi, r9
00007FF69FC22186  xor          edx, edx
00007FF69FC22188  nop          dword ptr [rax+rax*1], eax
00007FF69FC22190  lea          rcx, [r12+rdx*1]
00007FF69FC22194  cmp          r14, rcx
00007FF69FC22197  jbe          static void Fabled_Engine::main+1B07h (00007FF69FC22D17h)
00007FF69FC2219D  lea          rbp, [r11+rdi*1]
00007FF69FC221A1  mov          qword ptr [rsi+rdx*8-28h], rbp
00007FF69FC221A6  lea          rcx, [r12+rdx*1]
00007FF69FC221AA  add          rcx, 1h
00007FF69FC221AE  cmp          r14, rcx
00007FF69FC221B1  jbe          static void Fabled_Engine::main+1B18h (00007FF69FC22D28h)
00007FF69FC221B7  mov          rcx, qword ptr [rsp+38h]
00007FF69FC221BC  lea          rbx, [rcx+rdi*1]
00007FF69FC221C0  add          rbx, 2h
00007FF69FC221C4  mov          qword ptr [rsi+rdx*8-20h], rbx
00007FF69FC221C9  lea          rcx, [r12+rdx*1]
00007FF69FC221CD  add          rcx, 2h
00007FF69FC221D1  cmp          r14, rcx
00007FF69FC221D4  jbe          static void Fabled_Engine::main+1B29h (00007FF69FC22D39h)
00007FF69FC221DA  lea          rcx, [r11+rdi*1]
00007FF69FC221DE  add          rcx, 1h
00007FF69FC221E2  mov          qword ptr [rsi+rdx*8-18h], rcx
00007FF69FC221E7  lea          rcx, [r12+rdx*1]
00007FF69FC221EB  add          rcx, 3h
00007FF69FC221EF  cmp          r14, rcx
00007FF69FC221F2  jbe          static void Fabled_Engine::main+1B3Ah (00007FF69FC22D4Ah)
00007FF69FC221F8  mov          qword ptr [rsi+rdx*8-10h], rbp
00007FF69FC221FD  lea          rcx, [r12+rdx*1]
00007FF69FC22201  add          rcx, 4h
00007FF69FC22205  cmp          r14, rcx
00007FF69FC22208  jbe          static void Fabled_Engine::main+1B4Bh (00007FF69FC22D5Bh)
00007FF69FC2220E  mov          rcx, qword ptr [rsp+38h]
00007FF69FC22213  add          rcx, rdi
00007FF69FC22216  add          rcx, 1h
00007FF69FC2221A  mov          qword ptr [rsi+rdx*8-8h], rcx
00007FF69FC2221F  lea          rcx, [r12+rdx*1]
00007FF69FC22223  add          rcx, 5h
00007FF69FC22227  cmp          r14, rcx
00007FF69FC2222A  jbe          static void Fabled_Engine::main+1B5Ch (00007FF69FC22D6Ch)
00007FF69FC22230  lea          rcx, [r15+rdx*1]
00007FF69FC22234  mov          qword ptr [rsi+rdx*8], rbx
00007FF69FC22238  cmp          r14, rcx
00007FF69FC2223B  jbe          static void Fabled_Engine::main+1B6Dh (00007FF69FC22D7Dh)
00007FF69FC22241  mov          rcx, qword ptr [rsp+40h]
00007FF69FC22246  lea          rbp, [rcx+rdi*1]
00007FF69FC2224A  add          rbp, 1h
00007FF69FC2224E  mov          qword ptr [rax+rdx*8-28h], rbp
00007FF69FC22253  lea          rcx, [r15+rdx*1]
00007FF69FC22257  add          rcx, 1h
00007FF69FC2225B  cmp          r14, rcx
00007FF69FC2225E  jbe          static void Fabled_Engine::main+1B7Eh (00007FF69FC22D8Eh)
00007FF69FC22264  mov          rcx, qword ptr [rsp+50h]
00007FF69FC22269  lea          rbx, [rcx+rdi*1]
00007FF69FC2226D  add          rbx, 3h
00007FF69FC22271  mov          qword ptr [rax+rdx*8-20h], rbx
00007FF69FC22276  lea          rcx, [r15+rdx*1]
00007FF69FC2227A  add          rcx, 2h
00007FF69FC2227E  cmp          r14, rcx
00007FF69FC22281  jbe          static void Fabled_Engine::main+1B8Fh (00007FF69FC22D9Fh)
00007FF69FC22287  mov          rcx, qword ptr [rsp+40h]
00007FF69FC2228C  add          rcx, rdi
00007FF69FC2228F  add          rcx, 2h
00007FF69FC22293  mov          qword ptr [rax+rdx*8-18h], rcx
00007FF69FC22298  lea          rcx, [r15+rdx*1]
00007FF69FC2229C  add          rcx, 3h
00007FF69FC222A0  cmp          r14, rcx
00007FF69FC222A3  jbe          static void Fabled_Engine::main+1BA0h (00007FF69FC22DB0h)
00007FF69FC222A9  mov          qword ptr [rax+rdx*8-10h], rbp
00007FF69FC222AE  lea          rcx, [r15+rdx*1]
00007FF69FC222B2  add          rcx, 4h
00007FF69FC222B6  cmp          r14, rcx
00007FF69FC222B9  jbe          static void Fabled_Engine::main+1BB1h (00007FF69FC22DC1h)
00007FF69FC222BF  mov          rcx, qword ptr [rsp+50h]
00007FF69FC222C4  add          rcx, rdi
00007FF69FC222C7  add          rcx, 2h
00007FF69FC222CB  mov          qword ptr [rax+rdx*8-8h], rcx
00007FF69FC222D0  lea          rcx, [r15+rdx*1]
00007FF69FC222D4  add          rcx, 5h
00007FF69FC222D8  cmp          r14, rcx
00007FF69FC222DB  jbe          static void Fabled_Engine::main+1BC2h (00007FF69FC22DD2h)
00007FF69FC222E1  mov          qword ptr [rax+rdx*8], rbx
00007FF69FC222E5  add          rdx, 6h
00007FF69FC222E9  add          rdi, 1h
00007FF69FC222ED  cmp          qword ptr [rsp+B0h], rdx
00007FF69FC222F5  jne          static void Fabled_Engine::main+F80h (00007FF69FC22190h)
00007FF69FC222FB  add          r15, rdx
00007FF69FC222FE  add          r12, rdx
00007FF69FC22301  jmp          static void Fabled_Engine::main+F50h (00007FF69FC22160h)
00007FF69FC22306  cmp          byte ptr [rsp+48h], 0h
00007FF69FC2230B  mov          r11, qword ptr [rsp+28h]
00007FF69FC22310  mov          r15, qword ptr [rsp+160h]
00007FF69FC22318  mov          r12, qword ptr [rsp+1A0h]
00007FF69FC22320  je           static void Fabled_Engine::main+120Dh (00007FF69FC2241Dh)
00007FF69FC22326  mov          rax, qword ptr [rsp+1D8h]
00007FF69FC2232E  lea          r10, [rax+r11*2]
00007FF69FC22332  add          r10, 1h
00007FF69FC22336  mov          rax, qword ptr [rsp+30h]
00007FF69FC2233B  lea          r8, [rax+28h]
00007FF69FC2233F  xor          r9d, r9d
00007FF69FC22342  nop          word ptr cs:[rax+rax*1], ax
00007FF69FC2234C  nop          dword ptr [rax], eax
00007FF69FC22350  lea          rsi, [r8+r13*8]
00007FF69FC22354  mov          rdx, r10
00007FF69FC22357  mov          rbp, r12
00007FF69FC2235A  xor          edi, edi
00007FF69FC2235C  nop          dword ptr [rax], eax
00007FF69FC22360  lea          rcx, [rdi+r13*1]
00007FF69FC22364  cmp          r14, rcx
00007FF69FC22367  jbe          static void Fabled_Engine::main+1BD3h (00007FF69FC22DE3h)
00007FF69FC2236D  mov          qword ptr [rsi+rdi*8-28h], rbp
00007FF69FC22372  lea          rcx, [rdi+r13*1]
00007FF69FC22376  add          rcx, 1h
00007FF69FC2237A  cmp          r14, rcx
00007FF69FC2237D  jbe          static void Fabled_Engine::main+1BE4h (00007FF69FC22DF4h)
00007FF69FC22383  lea          rax, [rdx+1h]
00007FF69FC22387  mov          qword ptr [rsi+rdi*8-20h], rax
00007FF69FC2238C  lea          rcx, [rdi+r13*1]
00007FF69FC22390  add          rcx, 2h
00007FF69FC22394  cmp          r14, rcx
00007FF69FC22397  jbe          static void Fabled_Engine::main+1BF5h (00007FF69FC22E05h)
00007FF69FC2239D  lea          rbx, [rbp+1h]
00007FF69FC223A1  mov          qword ptr [rsi+rdi*8-18h], rbx
00007FF69FC223A6  lea          rcx, [rdi+r13*1]
00007FF69FC223AA  add          rcx, 3h
00007FF69FC223AE  cmp          r14, rcx
00007FF69FC223B1  jbe          static void Fabled_Engine::main+1C06h (00007FF69FC22E16h)
00007FF69FC223B7  mov          qword ptr [rsi+rdi*8-10h], rbp
00007FF69FC223BC  lea          rcx, [rdi+r13*1]
00007FF69FC223C0  add          rcx, 4h
00007FF69FC223C4  cmp          r14, rcx
00007FF69FC223C7  jbe          static void Fabled_Engine::main+1C17h (00007FF69FC22E27h)
00007FF69FC223CD  mov          qword ptr [rsi+rdi*8-8h], rdx
00007FF69FC223D2  lea          rcx, [rdi+r13*1]
00007FF69FC223D6  add          rcx, 5h
00007FF69FC223DA  cmp          r14, rcx
00007FF69FC223DD  jbe          static void Fabled_Engine::main+1C28h (00007FF69FC22E38h)
00007FF69FC223E3  mov          qword ptr [rsi+rdi*8], rax
00007FF69FC223E7  add          rdi, 6h
00007FF69FC223EB  mov          rdx, rax
00007FF69FC223EE  mov          rbp, rbx
00007FF69FC223F1  cmp          qword ptr [rsp+B0h], rdi
00007FF69FC223F9  jne          static void Fabled_Engine::main+1150h (00007FF69FC22360h)
00007FF69FC223FF  lea          rax, [r9+1h]
00007FF69FC22403  mov          rcx, qword ptr [rsp+58h]
00007FF69FC22408  add          r12, rcx
00007FF69FC2240B  add          r10, rcx
00007FF69FC2240E  add          r13, rdi
00007FF69FC22411  cmp          r9, r15
00007FF69FC22414  mov          r9, rax
00007FF69FC22417  jne          static void Fabled_Engine::main+1140h (00007FF69FC22350h)
00007FF69FC2241D  mov          qword ptr [rsp+1B0h], 0h
00007FF69FC22429  mov          dword ptr [rsp+1B8h], 0h
00007FF69FC22434  mov          qword ptr [rsp+1C0h], 0h
00007FF69FC22440  mov          dword ptr [rsp+1C8h], 0h
00007FF69FC2244B  xorps        xmm0, xmm0
00007FF69FC2244E  movaps       xmmword ptr [rsp+270h], xmm0
00007FF69FC22456  movaps       xmmword ptr [rsp+260h], xmm0
00007FF69FC2245E  mov          ecx, 40h
00007FF69FC22463  mov          rdi, qword ptr [rsp+1D0h]
00007FF69FC2246B  mov          rax, rdi
00007FF69FC2246E  mul          rcx
00007FF69FC22471  jo           static void Fabled_Engine::main+1DE6h (00007FF69FC22FF6h)
00007FF69FC22477  mov          rsi, rax
00007FF69FC2247A  test         rax, rax
00007FF69FC2247D  je           static void Fabled_Engine::main+1386h (00007FF69FC22596h)
00007FF69FC22483  mov          rcx, qword ptr [00007FF69FC451C8h]
00007FF69FC2248A  test         rcx, rcx
00007FF69FC2248D  jne          static void Fabled_Engine::main+1293h (00007FF69FC224A3h)
00007FF69FC2248F  call         GetProcessHeap (00007FF69FC3AA6Ch)
00007FF69FC22494  test         rax, rax
00007FF69FC22497  je           static void Fabled_Engine::main+12ABh (00007FF69FC224BBh)
00007FF69FC22499  mov          rcx, rax
00007FF69FC2249C  mov          qword ptr [00007FF69FC451C8h], rax
00007FF69FC224A3  xor          edx, edx
00007FF69FC224A5  mov          r8, rsi
00007FF69FC224A8  call         HeapAlloc (00007FF69FC3AA72h)
00007FF69FC224AD  test         rax, rax
00007FF69FC224B0  mov          r11, qword ptr [rsp+28h]
00007FF69FC224B5  jne          static void Fabled_Engine::main+138Bh (00007FF69FC2259Bh)
00007FF69FC224BB  mov          edx, 10h
00007FF69FC224C0  mov          rcx, rsi
00007FF69FC224C3  call         static void alloc::alloc::handle_alloc_error (00007FF69FC3BC90h)
00007FF69FC224C8  ud2
00007FF69FC224CA  lea          r8, [00007FF69FC3FC88h]
00007FF69FC224D1  mov          rcx, rax
00007FF69FC224D4  mov          rdx, r11
00007FF69FC224D7  call         static void core::panicking::panic_bounds_check (00007FF69FC3BD90h)
00007FF69FC224DC  ud2
00007FF69FC224DE  lea          r8, [00007FF69FC3FCA0h]
00007FF69FC224E5  call         static void core::panicking::panic_bounds_check (00007FF69FC3BD90h)
00007FF69FC224EA  ud2
00007FF69FC224EC  lea          r8, [00007FF69FC3FCB8h]
00007FF69FC224F3  mov          rdx, r10
00007FF69FC224F6  call         static void core::panicking::panic_bounds_check (00007FF69FC3BD90h)
00007FF69FC224FB  ud2
00007FF69FC224FD  lea          r8, [00007FF69FC3FCD0h]
00007FF69FC22504  mov          rdx, rax
00007FF69FC22507  call         static void core::panicking::panic_bounds_check (00007FF69FC3BD90h)
00007FF69FC2250C  ud2
00007FF69FC2250E  lea          r8, [00007FF69FC3FCE8h]
00007FF69FC22515  call         static void core::panicking::panic_bounds_check (00007FF69FC3BD90h)
00007FF69FC2251A  ud2
00007FF69FC2251C  lea          r8, [00007FF69FC3FD00h]
00007FF69FC22523  mov          rdx, r10
00007FF69FC22526  call         static void core::panicking::panic_bounds_check (00007FF69FC3BD90h)
00007FF69FC2252B  ud2
00007FF69FC2252D  lea          r8, [00007FF69FC3FD18h]
00007FF69FC22534  mov          rdx, rax
00007FF69FC22537  call         static void core::panicking::panic_bounds_check (00007FF69FC3BD90h)
00007FF69FC2253C  ud2
00007FF69FC2253E  lea          r8, [00007FF69FC3FD48h]
00007FF69FC22545  mov          rcx, rdx
00007FF69FC22548  mov          rdx, r10
00007FF69FC2254B  call         static void core::panicking::panic_bounds_check (00007FF69FC3BD90h)
00007FF69FC22550  ud2
00007FF69FC22552  lea          r8, [00007FF69FC3FD60h]
00007FF69FC22559  mov          rcx, rdx
00007FF69FC2255C  mov          rdx, r12
00007FF69FC2255F  call         static void core::panicking::panic_bounds_check (00007FF69FC3BD90h)
00007FF69FC22564  ud2
00007FF69FC22566  lea          r8, [00007FF69FC3FD78h]
00007FF69FC2256D  mov          rdx, rax
00007FF69FC22570  call         static void core::panicking::panic_bounds_check (00007FF69FC3BD90h)
00007FF69FC22575  ud2
00007FF69FC22577  lea          r8, [00007FF69FC3FD90h]
00007FF69FC2257E  mov          rdx, r9
00007FF69FC22581  call         static void core::panicking::panic_bounds_check (00007FF69FC3BD90h)
00007FF69FC22586  ud2
00007FF69FC22588  lea          r8, [00007FF69FC3FDA8h]
00007FF69FC2258F  call         static void core::panicking::panic_bounds_check (00007FF69FC3BD90h)
00007FF69FC22594  ud2
00007FF69FC22596  mov          eax, 10h
00007FF69FC2259B  shr          rsi, 6h
00007FF69FC2259F  mov          qword ptr [rsp+100h], rax
00007FF69FC225A7  mov          qword ptr [rsp+108h], rsi
00007FF69FC225AF  mov          qword ptr [rsp+110h], 0h
00007FF69FC225BB  mov          rcx, qword ptr [rsp+1B0h]
00007FF69FC225C3  mov          qword ptr [rsp+140h], rcx
00007FF69FC225CB  mov          ecx, dword ptr [rsp+1B8h]
00007FF69FC225D2  mov          dword ptr [rsp+148h], ecx
00007FF69FC225D9  mov          rcx, qword ptr [rsp+1C0h]
00007FF69FC225E1  mov          qword ptr [rsp+178h], rcx
00007FF69FC225E9  mov          ecx, dword ptr [rsp+1C8h]
00007FF69FC225F0  mov          dword ptr [rsp+180h], ecx
00007FF69FC225F7  movaps       xmm0, xmmword ptr [rsp+270h]
00007FF69FC225FF  movaps       xmmword ptr [rsp+210h], xmm0
00007FF69FC22607  movaps       xmm0, xmmword ptr [rsp+260h]
00007FF69FC2260F  movaps       xmmword ptr [rsp+200h], xmm0
00007FF69FC22617  cmp          rsi, rdi
00007FF69FC2261A  jb           static void Fabled_Engine::main+1C39h (00007FF69FC22E49h)
00007FF69FC22620  xor          ecx, ecx
00007FF69FC22622  mov          rdx, rcx
00007FF69FC22625  shl          rdx, 6h
00007FF69FC22629  add          rdx, rax
00007FF69FC2262C  cmp          rdi, 2h
00007FF69FC22630  jb           static void Fabled_Engine::main+149Ch (00007FF69FC226ACh)
00007FF69FC22632  lea          rax, [r11+r11*4]
00007FF69FC22636  mov          rbp, qword ptr [rsp+158h]
00007FF69FC2263E  add          rax, rbp
00007FF69FC22641  add          rax, 2h
00007FF69FC22645  nop          word ptr cs:[rax+rax*1], ax
00007FF69FC2264F  nop
00007FF69FC22650  mov          ebp, dword ptr [rsp+148h]
00007FF69FC22657  mov          dword ptr [rdx+8h], ebp
00007FF69FC2265A  mov          rbp, qword ptr [rsp+140h]
00007FF69FC22662  mov          qword ptr [rdx], rbp
00007FF69FC22665  mov          qword ptr [rdx+Ch], 0h
00007FF69FC2266D  mov          ebp, dword ptr [rsp+180h]
00007FF69FC22674  mov          dword ptr [rdx+1Ch], ebp
00007FF69FC22677  mov          rbp, qword ptr [rsp+178h]
00007FF69FC2267F  mov          qword ptr [rdx+14h], rbp
00007FF69FC22683  movaps       xmm0, xmmword ptr [rsp+210h]
00007FF69FC2268B  movaps       xmmword ptr [rdx+20h], xmm0
00007FF69FC2268F  movaps       xmm0, xmmword ptr [rsp+200h]
00007FF69FC22697  movaps       xmmword ptr [rdx+30h], xmm0
00007FF69FC2269B  add          rdx, 40h
00007FF69FC2269F  add          rax, FFFFFFFFFFFFFFFFh
00007FF69FC226A3  jne          static void Fabled_Engine::main+1440h (00007FF69FC22650h)
00007FF69FC226A5  add          rcx, rdi
00007FF69FC226A8  add          rcx, FFFFFFFFFFFFFFFFh
00007FF69FC226AC  test         rdi, rdi
00007FF69FC226AF  je           static void Fabled_Engine::main+14F0h (00007FF69FC22700h)
00007FF69FC226B1  mov          eax, dword ptr [rsp+148h]
00007FF69FC226B8  mov          dword ptr [rdx+8h], eax
00007FF69FC226BB  mov          rax, qword ptr [rsp+140h]
00007FF69FC226C3  mov          qword ptr [rdx], rax
00007FF69FC226C6  mov          qword ptr [rdx+Ch], 0h
00007FF69FC226CE  mov          eax, dword ptr [rsp+180h]
00007FF69FC226D5  mov          dword ptr [rdx+1Ch], eax
00007FF69FC226D8  mov          rax, qword ptr [rsp+178h]
00007FF69FC226E0  mov          qword ptr [rdx+14h], rax
00007FF69FC226E4  movaps       xmm0, xmmword ptr [rsp+210h]
00007FF69FC226EC  movaps       xmmword ptr [rdx+20h], xmm0
00007FF69FC226F0  movaps       xmm0, xmmword ptr [rsp+200h]
00007FF69FC226F8  movaps       xmmword ptr [rdx+30h], xmm0
00007FF69FC226FC  add          rcx, 1h
00007FF69FC22700  mov          qword ptr [rsp+110h], rcx
00007FF69FC22708  test         rdi, rdi
00007FF69FC2270B  je           static void Fabled_Engine::main+15C0h (00007FF69FC227D0h)
00007FF69FC22711  mov          r8, qword ptr [rsp+D0h]
00007FF69FC22719  mov          r9, qword ptr [rsp+E0h]
00007FF69FC22721  mov          rdi, qword ptr [rsp+100h]
00007FF69FC22729  mov          r10, qword ptr [rsp+198h]
00007FF69FC22731  lea          rax, [r11+r11*4]
00007FF69FC22735  mov          rdx, qword ptr [rsp+158h]
00007FF69FC2273D  lea          r11, [rdx+rax*1]
00007FF69FC22741  add          r11, 3h
00007FF69FC22745  mov          esi, 4h
00007FF69FC2274A  xor          ebx, ebx
00007FF69FC2274C  xorps        xmm0, xmm0
00007FF69FC2274F  nop
00007FF69FC22750  cmp          r10, rbx
00007FF69FC22753  je           static void Fabled_Engine::main+1D35h (00007FF69FC22F45h)
00007FF69FC22759  cmp          r9, rbx
00007FF69FC2275C  je           static void Fabled_Engine::main+1D49h (00007FF69FC22F59h)
00007FF69FC22762  mov          rdx, qword ptr [rsp+F8h]
00007FF69FC2276A  cmp          rdx, rbx
00007FF69FC2276D  jbe          static void Fabled_Engine::main+1D5Dh (00007FF69FC22F6Dh)
00007FF69FC22773  cmp          rcx, rbx
00007FF69FC22776  je           static void Fabled_Engine::main+1D6Eh (00007FF69FC22F7Eh)
00007FF69FC2277C  mov          rdx, qword ptr [rsp+188h]
00007FF69FC22784  mov          r15d, dword ptr [rdx+rsi*2]
00007FF69FC22788  mov          rdx, qword ptr [rdx+rsi*2-8h]
00007FF69FC2278D  mov          r12, qword ptr [r8+rbx*8]
00007FF69FC22791  add          rbx, 1h
00007FF69FC22795  mov          rbp, qword ptr [rsp+E8h]
00007FF69FC2279D  mov          rax, qword ptr [rbp+rsi*2-8h]
00007FF69FC227A2  mov          ebp, dword ptr [rbp+rsi*2]
00007FF69FC227A6  mov          qword ptr [rdi+rsi*8-20h], rdx
00007FF69FC227AB  mov          dword ptr [rdi+rsi*8-18h], r15d
00007FF69FC227B0  mov          qword ptr [rdi+rsi*8-14h], r12
00007FF69FC227B5  mov          dword ptr [rdi+rsi*8-4h], ebp
00007FF69FC227B9  mov          qword ptr [rdi+rsi*8-Ch], rax
00007FF69FC227BE  movaps       xmmword ptr [rdi+rsi*8+10h], xmm0
00007FF69FC227C3  movaps       xmmword ptr [rdi+rsi*8], xmm0
00007FF69FC227C7  add          rsi, 8h
00007FF69FC227CB  cmp          r11, rbx
00007FF69FC227CE  jne          static void Fabled_Engine::main+1540h (00007FF69FC22750h)
00007FF69FC227D0  mov          rcx, qword ptr [00007FF69FC451C8h]
00007FF69FC227D7  test         rcx, rcx
00007FF69FC227DA  je           static void Fabled_Engine::main+15D6h (00007FF69FC227E6h)
00007FF69FC227DC  mov          rdi, qword ptr [rsp+130h]
00007FF69FC227E4  jmp          static void Fabled_Engine::main+15F6h (00007FF69FC22806h)
00007FF69FC227E6  call         GetProcessHeap (00007FF69FC3AA6Ch)
00007FF69FC227EB  test         rax, rax
00007FF69FC227EE  mov          rdi, qword ptr [rsp+130h]
00007FF69FC227F6  je           static void Fabled_Engine::main+1DFCh (00007FF69FC2300Ch)
00007FF69FC227FC  mov          rcx, rax
00007FF69FC227FF  mov          qword ptr [00007FF69FC451C8h], rax
00007FF69FC22806  mov          r8d, 40h
00007FF69FC2280C  xor          edx, edx
00007FF69FC2280E  call         HeapAlloc (00007FF69FC3AA72h)
00007FF69FC22813  test         rax, rax
00007FF69FC22816  je           static void Fabled_Engine::main+1DFCh (00007FF69FC2300Ch)
00007FF69FC2281C  mov          rsi, rax
00007FF69FC2281F  shr          rdi, 3h
00007FF69FC22823  mov          rax, qword ptr [rsp+110h]
00007FF69FC2282B  mov          qword ptr [rsi+10h], rax
00007FF69FC2282F  movups       xmm0, xmmword ptr [rsp+100h]
00007FF69FC22837  movups       xmmword ptr [rsi], xmm0
00007FF69FC2283A  mov          rax, qword ptr [rsp+30h]
00007FF69FC2283F  mov          qword ptr [rsi+20h], rax
00007FF69FC22843  mov          qword ptr [rsi+28h], rdi
00007FF69FC22847  mov          qword ptr [rsi+30h], r14
00007FF69FC2284B  mov          dword ptr [rsi+18h], 0h
00007FF69FC22852  mov          rcx, qword ptr [00007FF69FC451C8h]
00007FF69FC22859  xor          edx, edx
00007FF69FC2285B  mov          r8, qword ptr [rsp+88h]
00007FF69FC22863  call         HeapFree (00007FF69FC3AA78h)
00007FF69FC22868  mov          rax, qword ptr [rsp+98h]
00007FF69FC22870  shl          rax, 3h
00007FF69FC22874  test         rax, rax
00007FF69FC22877  je           static void Fabled_Engine::main+167Fh (00007FF69FC2288Fh)
00007FF69FC22879  mov          r8, qword ptr [rsp+90h]
00007FF69FC22881  mov          rcx, qword ptr [00007FF69FC451C8h]
00007FF69FC22888  xor          edx, edx
00007FF69FC2288A  call         HeapFree (00007FF69FC3AA78h)
00007FF69FC2288F  mov          rax, qword ptr [rsp+68h]
00007FF69FC22894  shl          rax, 3h
00007FF69FC22898  test         rax, rax
00007FF69FC2289B  lea          rbx, [rsp+D0h]
00007FF69FC228A3  je           static void Fabled_Engine::main+16A8h (00007FF69FC228B8h)
00007FF69FC228A5  mov          r8, qword ptr [rsp+60h]
00007FF69FC228AA  mov          rcx, qword ptr [00007FF69FC451C8h]
00007FF69FC228B1  xor          edx, edx
00007FF69FC228B3  call         HeapFree (00007FF69FC3AA78h)
00007FF69FC228B8  mov          rax, qword ptr [rsp+F0h]
00007FF69FC228C0  shl          rax, 4h
00007FF69FC228C4  je           static void Fabled_Engine::main+16CCh (00007FF69FC228DCh)
00007FF69FC228C6  mov          r8, qword ptr [rsp+E8h]
00007FF69FC228CE  mov          rcx, qword ptr [00007FF69FC451C8h]
00007FF69FC228D5  xor          edx, edx
00007FF69FC228D7  call         HeapFree (00007FF69FC3AA78h)
00007FF69FC228DC  mov          rax, qword ptr [rsp+D8h]
00007FF69FC228E4  shl          rax, 3h
00007FF69FC228E8  test         rax, rax
00007FF69FC228EB  je           static void Fabled_Engine::main+16F3h (00007FF69FC22903h)
00007FF69FC228ED  mov          r8, qword ptr [rsp+D0h]
00007FF69FC228F5  mov          rcx, qword ptr [00007FF69FC451C8h]
00007FF69FC228FC  xor          edx, edx
00007FF69FC228FE  call         HeapFree (00007FF69FC3AA78h)
00007FF69FC22903  mov          rax, qword ptr [rsp+190h]
00007FF69FC2290B  shl          rax, 4h
00007FF69FC2290F  je           static void Fabled_Engine::main+1717h (00007FF69FC22927h)
00007FF69FC22911  mov          r8, qword ptr [rsp+188h]
00007FF69FC22919  mov          rcx, qword ptr [00007FF69FC451C8h]
00007FF69FC22920  xor          edx, edx
00007FF69FC22922  call         HeapFree (00007FF69FC3AA78h)

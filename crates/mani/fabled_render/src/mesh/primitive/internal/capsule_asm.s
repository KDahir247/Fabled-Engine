# --------------- Capsule Dissassembly -------------------
00007FF7AF061359  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF061360  test         rcx, rcx
00007FF7AF061363  jne          static void Fabled_Engine::main+BDh (00007FF7AF06137Dh)
00007FF7AF061365  call         GetProcessHeap (00007FF7AF079AA2h)
00007FF7AF06136A  test         rax, rax
00007FF7AF06136D  je           static void Fabled_Engine::main+1452h (00007FF7AF062712h)
00007FF7AF061373  mov          rcx, rax
00007FF7AF061376  mov          qword ptr [00007FF7AF0841C8h], rax
00007FF7AF06137D  mov          r8d, B90h
00007FF7AF061383  xor          edx, edx
00007FF7AF061385  call         HeapAlloc (00007FF7AF079AA8h)
00007FF7AF06138A  test         rax, rax
00007FF7AF06138D  je           static void Fabled_Engine::main+1452h (00007FF7AF062712h)
00007FF7AF061393  mov          r13, rax
00007FF7AF061396  xor          eax, eax
00007FF7AF061398  xorps        xmm0, xmm0
00007FF7AF06139B  nop          dword ptr [rax+rax*1], eax
00007FF7AF0613A0  movaps       xmmword ptr [r13+rax*1], xmm0
00007FF7AF0613A6  movaps       xmmword ptr [r13+rax*1+10h], xmm0
00007FF7AF0613AC  movaps       xmmword ptr [r13+rax*1+20h], xmm0
00007FF7AF0613B2  movaps       xmmword ptr [r13+rax*1+30h], xmm0
00007FF7AF0613B8  movaps       xmmword ptr [r13+rax*1+40h], xmm0
00007FF7AF0613BE  movaps       xmmword ptr [r13+rax*1+50h], xmm0
00007FF7AF0613C4  movaps       xmmword ptr [r13+rax*1+60h], xmm0
00007FF7AF0613CA  movaps       xmmword ptr [r13+rax*1+70h], xmm0
00007FF7AF0613D0  sub          rax, FFFFFFFFFFFFFF80h
00007FF7AF0613D4  cmp          rax, B80h
00007FF7AF0613DA  jne          static void Fabled_Engine::main+E0h (00007FF7AF0613A0h)
00007FF7AF0613DC  xorps        xmm0, xmm0
00007FF7AF0613DF  movaps       xmmword ptr [r13+rax*1], xmm0
00007FF7AF0613E5  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF0613EC  test         rcx, rcx
00007FF7AF0613EF  jne          static void Fabled_Engine::main+149h (00007FF7AF061409h)
00007FF7AF0613F1  call         GetProcessHeap (00007FF7AF079AA2h)
00007FF7AF0613F6  test         rax, rax
00007FF7AF0613F9  je           static void Fabled_Engine::main+1460h (00007FF7AF062720h)
00007FF7AF0613FF  mov          rcx, rax
00007FF7AF061402  mov          qword ptr [00007FF7AF0841C8h], rax
00007FF7AF061409  mov          r8d, 5C8h
00007FF7AF06140F  xor          edx, edx
00007FF7AF061411  call         HeapAlloc (00007FF7AF079AA8h)
00007FF7AF061416  test         rax, rax
00007FF7AF061419  je           static void Fabled_Engine::main+1460h (00007FF7AF062720h)
00007FF7AF06141F  mov          r15, rax
00007FF7AF061422  xor          ecx, ecx
00007FF7AF061424  xorps        xmm0, xmm0
00007FF7AF061427  nop          word ptr [rax+rax*1], ax
00007FF7AF061430  mov          rax, rcx
00007FF7AF061433  movups       xmmword ptr [r15+rcx*8], xmm0
00007FF7AF061438  movups       xmmword ptr [r15+rcx*8+10h], xmm0
00007FF7AF06143E  movups       xmmword ptr [r15+rcx*8+20h], xmm0
00007FF7AF061444  movups       xmmword ptr [r15+rcx*8+30h], xmm0
00007FF7AF06144A  add          rcx, 8h
00007FF7AF06144E  cmp          rcx, B8h
00007FF7AF061455  jne          static void Fabled_Engine::main+170h (00007FF7AF061430h)
00007FF7AF061457  mov          qword ptr [r15+rax*8+40h], 0h
00007FF7AF061460  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF061467  test         rcx, rcx
00007FF7AF06146A  jne          static void Fabled_Engine::main+1C4h (00007FF7AF061484h)
00007FF7AF06146C  call         GetProcessHeap (00007FF7AF079AA2h)
00007FF7AF061471  test         rax, rax
00007FF7AF061474  je           static void Fabled_Engine::main+1452h (00007FF7AF062712h)
00007FF7AF06147A  mov          rcx, rax
00007FF7AF06147D  mov          qword ptr [00007FF7AF0841C8h], rax
00007FF7AF061484  mov          r8d, B90h
00007FF7AF06148A  xor          edx, edx
00007FF7AF06148C  call         HeapAlloc (00007FF7AF079AA8h)
00007FF7AF061491  test         rax, rax
00007FF7AF061494  je           static void Fabled_Engine::main+1452h (00007FF7AF062712h)
00007FF7AF06149A  mov          r12, rax
00007FF7AF06149D  xor          eax, eax
00007FF7AF06149F  xorps        xmm0, xmm0
00007FF7AF0614A2  nop          word ptr cs:[rax+rax*1], ax
00007FF7AF0614AC  nop          dword ptr [rax], eax
00007FF7AF0614B0  movaps       xmmword ptr [r12+rax*1], xmm0
00007FF7AF0614B5  movaps       xmmword ptr [r12+rax*1+10h], xmm0
00007FF7AF0614BB  movaps       xmmword ptr [r12+rax*1+20h], xmm0
00007FF7AF0614C1  movaps       xmmword ptr [r12+rax*1+30h], xmm0
00007FF7AF0614C7  movaps       xmmword ptr [r12+rax*1+40h], xmm0
00007FF7AF0614CD  movaps       xmmword ptr [r12+rax*1+50h], xmm0
00007FF7AF0614D3  movaps       xmmword ptr [r12+rax*1+60h], xmm0
00007FF7AF0614D9  movaps       xmmword ptr [r12+rax*1+70h], xmm0
00007FF7AF0614DF  sub          rax, FFFFFFFFFFFFFF80h
00007FF7AF0614E3  cmp          rax, B80h
00007FF7AF0614E9  jne          static void Fabled_Engine::main+1F0h (00007FF7AF0614B0h)
00007FF7AF0614EB  xorps        xmm0, xmm0
00007FF7AF0614EE  movaps       xmmword ptr [r12+rax*1], xmm0
00007FF7AF0614F3  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF0614FA  test         rcx, rcx
00007FF7AF0614FD  jne          static void Fabled_Engine::main+257h (00007FF7AF061517h)
00007FF7AF0614FF  call         GetProcessHeap (00007FF7AF079AA2h)
00007FF7AF061504  test         rax, rax
00007FF7AF061507  je           static void Fabled_Engine::main+1459h (00007FF7AF062719h)
00007FF7AF06150D  mov          rcx, rax
00007FF7AF061510  mov          qword ptr [00007FF7AF0841C8h], rax
00007FF7AF061517  mov          r8d, 80h
00007FF7AF06151D  xor          edx, edx
00007FF7AF06151F  call         HeapAlloc (00007FF7AF079AA8h)
00007FF7AF061524  test         rax, rax
00007FF7AF061527  je           static void Fabled_Engine::main+1459h (00007FF7AF062719h)
00007FF7AF06152D  mov          r14, rax
00007FF7AF061530  xorps        xmm0, xmm0
00007FF7AF061533  movups       xmmword ptr [rax], xmm0
00007FF7AF061536  movups       xmmword ptr [rax+10h], xmm0
00007FF7AF06153A  movups       xmmword ptr [rax+20h], xmm0
00007FF7AF06153E  movups       xmmword ptr [rax+30h], xmm0
00007FF7AF061542  movups       xmmword ptr [rax+40h], xmm0
00007FF7AF061546  movups       xmmword ptr [rax+50h], xmm0
00007FF7AF06154A  movups       xmmword ptr [rax+60h], xmm0
00007FF7AF06154E  movups       xmmword ptr [rax+70h], xmm0
00007FF7AF061552  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF061559  test         rcx, rcx
00007FF7AF06155C  jne          static void Fabled_Engine::main+2B6h (00007FF7AF061576h)
00007FF7AF06155E  call         GetProcessHeap (00007FF7AF079AA2h)
00007FF7AF061563  test         rax, rax
00007FF7AF061566  je           static void Fabled_Engine::main+1459h (00007FF7AF062719h)
00007FF7AF06156C  mov          rcx, rax
00007FF7AF06156F  mov          qword ptr [00007FF7AF0841C8h], rax
00007FF7AF061576  mov          r8d, 80h
00007FF7AF06157C  xor          edx, edx
00007FF7AF06157E  call         HeapAlloc (00007FF7AF079AA8h)
00007FF7AF061583  test         rax, rax
00007FF7AF061586  je           static void Fabled_Engine::main+1459h (00007FF7AF062719h)
00007FF7AF06158C  mov          rsi, rax
00007FF7AF06158F  xorps        xmm0, xmm0
00007FF7AF061592  movups       xmmword ptr [rax], xmm0
00007FF7AF061595  movups       xmmword ptr [rax+10h], xmm0
00007FF7AF061599  movups       xmmword ptr [rax+20h], xmm0
00007FF7AF06159D  movups       xmmword ptr [rax+30h], xmm0
00007FF7AF0615A1  movups       xmmword ptr [rax+40h], xmm0
00007FF7AF0615A5  movups       xmmword ptr [rax+50h], xmm0
00007FF7AF0615A9  movups       xmmword ptr [rax+60h], xmm0
00007FF7AF0615AD  movups       xmmword ptr [rax+70h], xmm0
00007FF7AF0615B1  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF0615B8  test         rcx, rcx
00007FF7AF0615BB  jne          static void Fabled_Engine::main+315h (00007FF7AF0615D5h)
00007FF7AF0615BD  call         GetProcessHeap (00007FF7AF079AA2h)
00007FF7AF0615C2  test         rax, rax
00007FF7AF0615C5  je           static void Fabled_Engine::main+1467h (00007FF7AF062727h)
00007FF7AF0615CB  mov          rcx, rax
00007FF7AF0615CE  mov          qword ptr [00007FF7AF0841C8h], rax
00007FF7AF0615D5  mov          r8d, 44h
00007FF7AF0615DB  mov          edx, 8h
00007FF7AF0615E0  call         HeapAlloc (00007FF7AF079AA8h)
00007FF7AF0615E5  mov          qword ptr [rsp+B8h], rax
00007FF7AF0615ED  test         rax, rax
00007FF7AF0615F0  je           static void Fabled_Engine::main+1467h (00007FF7AF062727h)
00007FF7AF0615F6  mov          qword ptr [rsp+70h], rsi
00007FF7AF0615FB  xor          esi, esi
00007FF7AF0615FD  movss        xmm10, dword ptr [__real@3f000000 (00007FF7AF07D530h)]
00007FF7AF061606  movss        xmm8, dword ptr [__real@bd800000 (00007FF7AF07D534h)]
00007FF7AF06160F  movss        xmm9, dword ptr [__real@3f800000 (00007FF7AF07D538h)]
00007FF7AF061618  movss        xmm11, dword ptr [__real@3ec90fdb (00007FF7AF07D53Ch)]
00007FF7AF061621  movaps       xmm12, xmmword ptr [__xmm@00000000000000003f80000000000000 (00007FF7AF07D540h)]
00007FF7AF061629  movaps       xmm13, xmmword ptr [__xmm@0000000000000000bf80000000000000 (00007FF7AF07D550h)]
00007FF7AF061631  xor          ebp, ebp
00007FF7AF061633  jmp          static void Fabled_Engine::main+437h (00007FF7AF0616F7h)
00007FF7AF061638  nop          dword ptr [rax+rax*1], eax
00007FF7AF061640  xorps        xmm7, xmm7
00007FF7AF061643  cvtsi2ss     xmm7, rbp
00007FF7AF061648  add          rbp, 1h
00007FF7AF06164C  movaps       xmm6, xmm7
00007FF7AF06164F  addss        xmm6, xmm10
00007FF7AF061654  mulss        xmm6, xmm8
00007FF7AF061659  addss        xmm6, xmm9
00007FF7AF06165E  mulss        xmm7, xmm11
00007FF7AF061663  movaps       xmm0, xmm7
00007FF7AF061666  call         sinf (00007FF7AF07ABC9h)
00007FF7AF06166B  movaps       xmm14, xmm0
00007FF7AF06166F  movaps       xmm0, xmm7
00007FF7AF061672  call         cosf (00007FF7AF07ABCFh)
00007FF7AF061677  movss        dword ptr [r14+rsi*1], xmm0
00007FF7AF06167D  movss        dword ptr [r14+rsi*1+4h], xmm14
00007FF7AF061684  mulss        xmm14, xmm10
00007FF7AF061689  mulss        xmm0, xmm10
00007FF7AF06168E  mov          rax, qword ptr [rsp+70h]
00007FF7AF061693  movss        dword ptr [rax+rsi*1], xmm0
00007FF7AF061698  movss        dword ptr [rax+rsi*1+4h], xmm14
00007FF7AF06169F  movaps       xmmword ptr [r13+rsi*2], xmm12
00007FF7AF0616A5  movss        dword ptr [r15+rsi*1], xmm6
00007FF7AF0616AB  mov          dword ptr [r15+rsi*1+4h], 3F800000h
00007FF7AF0616B4  movaps       xmmword ptr [r12+rsi*2], xmm12
00007FF7AF0616B9  xorps        xmm0, xmm0
00007FF7AF0616BC  subps        xmm0, xmmword ptr [r13+rsi*2]
00007FF7AF0616C2  movaps       xmmword ptr [r13+rsi*2+A90h], xmm0
00007FF7AF0616CB  movss        dword ptr [r15+rsi*1+548h], xmm6
00007FF7AF0616D5  mov          dword ptr [r15+rsi*1+54Ch], 0h
00007FF7AF0616E1  movaps       xmmword ptr [r12+rsi*2+A90h], xmm13
00007FF7AF0616EA  add          rsi, 8h
00007FF7AF0616EE  cmp          rsi, 80h
00007FF7AF0616F5  je           static void Fabled_Engine::main+45Fh (00007FF7AF06171Fh)
00007FF7AF0616F7  test         rbp, rbp
00007FF7AF0616FA  jns          static void Fabled_Engine::main+380h (00007FF7AF061640h)
00007FF7AF061700  mov          rax, rbp
00007FF7AF061703  shr          rax, 1h
00007FF7AF061706  mov          ecx, ebp
00007FF7AF061708  and          ecx, 1h
00007FF7AF06170B  or           rcx, rax
00007FF7AF06170E  xorps        xmm7, xmm7
00007FF7AF061711  cvtsi2ss     xmm7, rcx
00007FF7AF061716  addss        xmm7, xmm7
00007FF7AF06171A  jmp          static void Fabled_Engine::main+388h (00007FF7AF061648h)
00007FF7AF06171F  xor          eax, eax
00007FF7AF061721  movaps       xmm11, xmmword ptr [__xmm@80000000800000008000000080000000 (00007FF7AF07D560h)]
00007FF7AF061729  movss        xmm1, dword ptr [__real@3f000000 (00007FF7AF07D530h)]
00007FF7AF061731  movss        xmm2, dword ptr [__real@bf000000 (00007FF7AF07D570h)]
00007FF7AF061739  xor          ecx, ecx
00007FF7AF06173B  mov          rbx, qword ptr [rsp+70h]
00007FF7AF061740  mov          rdi, qword ptr [rsp+B8h]
00007FF7AF061748  jmp          static void Fabled_Engine::main+54Dh (00007FF7AF06180Dh)
00007FF7AF06174D  nop          dword ptr [rax], eax
00007FF7AF061750  xorps        xmm0, xmm0
00007FF7AF061753  cvtsi2ss     xmm0, rcx
00007FF7AF061758  lea          edx, [rcx*8]
00007FF7AF06175F  add          rcx, 1h
00007FF7AF061763  mulss        xmm0, xmm8
00007FF7AF061768  addss        xmm0, xmm9
00007FF7AF06176D  movss        dword ptr [rdi+rax*1], xmm0
00007FF7AF061772  and          edx, 78h
00007FF7AF061775  movss        xmm3, dword ptr [rbx+rdx*1]
00007FF7AF06177A  movss        xmm4, dword ptr [rbx+rdx*1+4h]
00007FF7AF061780  xorps        xmm4, xmm11
00007FF7AF061784  movaps       xmm5, xmm3
00007FF7AF061787  unpcklps     xmm5, xmm1
00007FF7AF06178A  shufps       xmm5, xmm4, 4h
00007FF7AF06178E  movss        xmm6, dword ptr [r14+rdx*1]
00007FF7AF061794  movss        xmm7, dword ptr [r14+rdx*1+4h]
00007FF7AF06179B  movaps       xmmword ptr [r13+rax*4+430h], xmm5
00007FF7AF0617A4  movss        dword ptr [r15+rax*2+218h], xmm0
00007FF7AF0617AE  mov          dword ptr [r15+rax*2+21Ch], 3F400000h
00007FF7AF0617BA  xorps        xmm7, xmm11
00007FF7AF0617BE  shufps       xmm6, xmm7, 4h
00007FF7AF0617C2  movaps       xmmword ptr [r12+rax*4+430h], xmm6
00007FF7AF0617CB  unpcklps     xmm3, xmm2
00007FF7AF0617CE  shufps       xmm3, xmm4, 4h
00007FF7AF0617D2  movaps       xmmword ptr [r13+rax*4+650h], xmm3
00007FF7AF0617DB  movss        dword ptr [r15+rax*2+328h], xmm0
00007FF7AF0617E5  mov          dword ptr [r15+rax*2+32Ch], 3E800000h
00007FF7AF0617F1  movaps       xmm0, xmmword ptr [r12+rax*4+430h]
00007FF7AF0617FA  movaps       xmmword ptr [r12+rax*4+650h], xmm0
00007FF7AF061803  add          rax, 4h
00007FF7AF061807  cmp          rax, 44h
00007FF7AF06180B  je           static void Fabled_Engine::main+575h (00007FF7AF061835h)
00007FF7AF06180D  test         rcx, rcx
00007FF7AF061810  jns          static void Fabled_Engine::main+490h (00007FF7AF061750h)
00007FF7AF061816  mov          rdx, rcx
00007FF7AF061819  shr          rdx, 1h
00007FF7AF06181C  mov          ebp, ecx
00007FF7AF06181E  and          ebp, 1h
00007FF7AF061821  or           rbp, rdx
00007FF7AF061824  xorps        xmm0, xmm0
00007FF7AF061827  cvtsi2ss     xmm0, rbp
00007FF7AF06182C  addss        xmm0, xmm0
00007FF7AF061830  jmp          static void Fabled_Engine::main+498h (00007FF7AF061758h)
00007FF7AF061835  xor          eax, eax
00007FF7AF061837  movss        xmm10, dword ptr [__real@be43ef16 (00007FF7AF07D574h)]
00007FF7AF061840  movss        xmm12, dword ptr [__real@3e43ef16 (00007FF7AF07D578h)]
00007FF7AF061849  movss        xmm13, dword ptr [__real@3f7641af (00007FF7AF07D57Ch)]
00007FF7AF061852  movss        xmm2, dword ptr [__real@bec3ef16 (00007FF7AF07D580h)]
00007FF7AF06185A  movss        xmm14, dword ptr [__real@3ec3ef16 (00007FF7AF07D584h)]
00007FF7AF061863  movss        xmm8, dword ptr [__real@3f6c835e (00007FF7AF07D588h)]
00007FF7AF06186C  movss        xmm1, dword ptr [__real@beec835e (00007FF7AF07D58Ch)]
00007FF7AF061874  movss        xmm15, dword ptr [__real@bf30fbc6 (00007FF7AF07D594h)]
00007FF7AF06187D  movss        xmm9, dword ptr [__real@bf6c835e (00007FF7AF07D598h)]
00007FF7AF061886  xor          edx, edx
00007FF7AF061888  nop          dword ptr [rax+rax*1], eax
00007FF7AF061890  lea          rcx, [rdx+1h]
00007FF7AF061894  movss        xmm7, dword ptr [rdi+rax*1]
00007FF7AF061899  and          edx, Fh
00007FF7AF06189C  movss        xmm5, dword ptr [r14+rdx*8]
00007FF7AF0618A2  movss        xmm3, dword ptr [r14+rdx*8+4h]
00007FF7AF0618A9  movaps       xmm6, xmm3
00007FF7AF0618AC  mulss        xmm6, xmm10
00007FF7AF0618B1  movaps       xmm0, xmm5
00007FF7AF0618B4  mulss        xmm0, xmm12
00007FF7AF0618B9  unpcklps     xmm0, xmm13
00007FF7AF0618BD  shufps       xmm0, xmm6, 4h
00007FF7AF0618C1  movaps       xmmword ptr [r13+rax*4+100h], xmm0
00007FF7AF0618CA  movss        dword ptr [r15+rax*2+80h], xmm7
00007FF7AF0618D4  mov          dword ptr [r15+rax*2+84h], 3F700000h
00007FF7AF0618E0  movaps       xmm0, xmm3
00007FF7AF0618E3  mulss        xmm0, xmm2
00007FF7AF0618E7  movaps       xmm6, xmm5
00007FF7AF0618EA  mulss        xmm6, xmm14
00007FF7AF0618EF  unpcklps     xmm6, xmm8
00007FF7AF0618F3  shufps       xmm6, xmm0, 4h
00007FF7AF0618F7  movaps       xmmword ptr [r12+rax*4+100h], xmm6
00007FF7AF061900  movaps       xmm0, xmm3
00007FF7AF061903  mulss        xmm0, xmm1
00007FF7AF061907  movaps       xmm6, xmm5
00007FF7AF06190A  mulss        xmm6, dword ptr [__real@3eec835e (00007FF7AF07D590h)]
00007FF7AF061912  unpcklps     xmm6, xmm15
00007FF7AF061916  shufps       xmm6, xmm0, 4h
00007FF7AF06191A  movaps       xmmword ptr [r13+rax*4+760h], xmm6
00007FF7AF061923  movss        dword ptr [r15+rax*2+3B0h], xmm7
00007FF7AF06192D  mov          dword ptr [r15+rax*2+3B4h], 3E400000h
00007FF7AF061939  mulss        xmm3, xmm9
00007FF7AF06193E  mulss        xmm5, xmm8
00007FF7AF061943  unpcklps     xmm5, xmm2
00007FF7AF061946  shufps       xmm5, xmm3, 4h
00007FF7AF06194A  movaps       xmmword ptr [r12+rax*4+760h], xmm5
00007FF7AF061953  add          rax, 4h
00007FF7AF061957  mov          rdx, rcx
00007FF7AF06195A  cmp          rax, 44h
00007FF7AF06195E  jne          static void Fabled_Engine::main+5D0h (00007FF7AF061890h)
00007FF7AF061964  movaps       xmm15, xmm1
00007FF7AF061968  xor          eax, eax
00007FF7AF06196A  movss        xmm10, dword ptr [__real@beb504f3 (00007FF7AF07D59Ch)]
00007FF7AF061973  movss        xmm12, dword ptr [__real@3eb504f3 (00007FF7AF07D5A0h)]
00007FF7AF06197C  movss        xmm13, dword ptr [__real@3f5a827a (00007FF7AF07D5A4h)]
00007FF7AF061985  movss        xmm5, dword ptr [__real@bf3504f3 (00007FF7AF07D5A8h)]
00007FF7AF06198D  movss        xmm6, dword ptr [__real@3f3504f3 (00007FF7AF07D5ACh)]
00007FF7AF061995  movss        xmm14, dword ptr [__real@bf5a827a (00007FF7AF07D5B0h)]
00007FF7AF06199E  xor          edx, edx
00007FF7AF0619A0  lea          rcx, [rdx+1h]
00007FF7AF0619A4  movss        xmm2, dword ptr [rdi+rax*1]
00007FF7AF0619A9  and          edx, Fh
00007FF7AF0619AC  movss        xmm4, dword ptr [r14+rdx*8]
00007FF7AF0619B2  movss        xmm3, dword ptr [r14+rdx*8+4h]
00007FF7AF0619B9  movaps       xmm7, xmm3
00007FF7AF0619BC  mulss        xmm7, xmm10
00007FF7AF0619C1  movaps       xmm1, xmm4
00007FF7AF0619C4  mulss        xmm1, xmm12
00007FF7AF0619C9  movaps       xmm0, xmm1
00007FF7AF0619CC  unpcklps     xmm0, xmm13
00007FF7AF0619D0  shufps       xmm0, xmm7, 4h
00007FF7AF0619D4  movaps       xmmword ptr [r13+rax*4+210h], xmm0
00007FF7AF0619DD  movss        dword ptr [r15+rax*2+108h], xmm2
00007FF7AF0619E7  mov          dword ptr [r15+rax*2+10Ch], 3F600000h
00007FF7AF0619F3  mulss        xmm3, xmm5
00007FF7AF0619F7  mulss        xmm4, xmm6
00007FF7AF0619FB  movaps       xmm0, xmm4
00007FF7AF0619FE  unpcklps     xmm0, xmm6
00007FF7AF061A01  shufps       xmm0, xmm3, 4h
00007FF7AF061A05  movaps       xmmword ptr [r12+rax*4+210h], xmm0
00007FF7AF061A0E  unpcklps     xmm1, xmm14
00007FF7AF061A12  shufps       xmm1, xmm7, 4h
00007FF7AF061A16  movaps       xmmword ptr [r13+rax*4+870h], xmm1
00007FF7AF061A1F  movss        dword ptr [r15+rax*2+438h], xmm2
00007FF7AF061A29  mov          dword ptr [r15+rax*2+43Ch], 3E000000h
00007FF7AF061A35  unpcklps     xmm4, xmm5
00007FF7AF061A38  shufps       xmm4, xmm3, 4h
00007FF7AF061A3C  movaps       xmmword ptr [r12+rax*4+870h], xmm4
00007FF7AF061A45  add          rax, 4h
00007FF7AF061A49  mov          rdx, rcx
00007FF7AF061A4C  cmp          rax, 44h
00007FF7AF061A50  jne          static void Fabled_Engine::main+6E0h (00007FF7AF0619A0h)
00007FF7AF061A56  xor          eax, eax
00007FF7AF061A58  movss        xmm10, dword ptr [__real@3f30fbc5 (00007FF7AF07D5B4h)]
00007FF7AF061A61  movss        xmm3, dword ptr [__real@3ec3ef15 (00007FF7AF07D5B8h)]
00007FF7AF061A69  movss        xmm12, dword ptr [__real@be43ef15 (00007FF7AF07D5BCh)]
00007FF7AF061A72  movss        xmm13, dword ptr [__real@3e43ef15 (00007FF7AF07D5C0h)]
00007FF7AF061A7B  movss        xmm14, dword ptr [__real@bf7641af (00007FF7AF07D5C4h)]
00007FF7AF061A84  movss        xmm0, dword ptr [__real@bec3ef15 (00007FF7AF07D5C8h)]
00007FF7AF061A8C  xor          edx, edx
00007FF7AF061A8E  movss        xmm7, dword ptr [__real@3eec835e (00007FF7AF07D590h)]
00007FF7AF061A96  nop          word ptr cs:[rax+rax*1], ax
00007FF7AF061AA0  lea          rcx, [rdx+1h]
00007FF7AF061AA4  movss        xmm6, dword ptr [rdi+rax*1]
00007FF7AF061AA9  and          edx, Fh
00007FF7AF061AAC  movss        xmm2, dword ptr [r14+rdx*8]
00007FF7AF061AB2  movss        xmm5, dword ptr [r14+rdx*8+4h]
00007FF7AF061AB9  movaps       xmm1, xmm5
00007FF7AF061ABC  mulss        xmm1, xmm15
00007FF7AF061AC1  movaps       xmm4, xmm2
00007FF7AF061AC4  mulss        xmm4, xmm7
00007FF7AF061AC8  unpcklps     xmm4, xmm10
00007FF7AF061ACC  shufps       xmm4, xmm1, 4h
00007FF7AF061AD0  movaps       xmmword ptr [r13+rax*4+320h], xmm4
00007FF7AF061AD9  movss        dword ptr [r15+rax*2+190h], xmm6
00007FF7AF061AE3  mov          dword ptr [r15+rax*2+194h], 3F500000h
00007FF7AF061AEF  movaps       xmm1, xmm5
00007FF7AF061AF2  mulss        xmm1, xmm9
00007FF7AF061AF7  movaps       xmm4, xmm2
00007FF7AF061AFA  mulss        xmm4, xmm8
00007FF7AF061AFF  unpcklps     xmm4, xmm3
00007FF7AF061B02  shufps       xmm4, xmm1, 4h
00007FF7AF061B06  movaps       xmmword ptr [r12+rax*4+320h], xmm4
00007FF7AF061B0F  movaps       xmm1, xmm5
00007FF7AF061B12  mulss        xmm1, xmm12
00007FF7AF061B17  movaps       xmm4, xmm2
00007FF7AF061B1A  mulss        xmm4, xmm13
00007FF7AF061B1F  unpcklps     xmm4, xmm14
00007FF7AF061B23  shufps       xmm4, xmm1, 4h
00007FF7AF061B27  movaps       xmmword ptr [r13+rax*4+980h], xmm4
00007FF7AF061B30  movss        dword ptr [r15+rax*2+4C0h], xmm6
00007FF7AF061B3A  mov          dword ptr [r15+rax*2+4C4h], 3D800000h
00007FF7AF061B46  mulss        xmm5, xmm0
00007FF7AF061B4A  mulss        xmm2, xmm3
00007FF7AF061B4E  unpcklps     xmm2, xmm9
00007FF7AF061B52  shufps       xmm2, xmm5, 4h
00007FF7AF061B56  movaps       xmmword ptr [r12+rax*4+980h], xmm2
00007FF7AF061B5F  add          rax, 4h
00007FF7AF061B63  mov          rdx, rcx
00007FF7AF061B66  cmp          rax, 44h
00007FF7AF061B6A  jne          static void Fabled_Engine::main+7E0h (00007FF7AF061AA0h)
00007FF7AF061B70  mov          ecx, 54h
00007FF7AF061B75  xor          eax, eax
00007FF7AF061B77  nop          word ptr [rax+rax*1], ax
00007FF7AF061B80  cmp          rax, 194h
00007FF7AF061B86  je           static void Fabled_Engine::main+143Fh (00007FF7AF0626FFh)
00007FF7AF061B8C  lea          edx, [rcx-54h]
00007FF7AF061B8F  and          edx, Fh
00007FF7AF061B92  movss        xmm0, dword ptr [rbx+rdx*8+4h]
00007FF7AF061B98  xorps        xmm0, xmm11
00007FF7AF061B9C  movss        xmm1, dword ptr [rdi+rax*1]
00007FF7AF061BA1  movss        xmm2, dword ptr [rbx+rdx*8]
00007FF7AF061BA6  shufps       xmm2, xmm0, 4h
00007FF7AF061BAA  movss        xmm0, dword ptr [r14+rdx*8]
00007FF7AF061BB0  movss        xmm3, dword ptr [r14+rdx*8+4h]
00007FF7AF061BB7  movaps       xmmword ptr [r13+rax*4+540h], xmm2
00007FF7AF061BC0  movss        dword ptr [r15+rax*2+2A0h], xmm1
00007FF7AF061BCA  mov          dword ptr [r15+rax*2+2A4h], 3F000000h
00007FF7AF061BD6  xorps        xmm3, xmm11
00007FF7AF061BDA  shufps       xmm0, xmm3, 4h
00007FF7AF061BDE  movaps       xmmword ptr [r12+rax*4+540h], xmm0
00007FF7AF061BE7  add          rcx, 1h
00007FF7AF061BEB  add          rax, 4h
00007FF7AF061BEF  cmp          rax, 44h
00007FF7AF061BF3  jne          static void Fabled_Engine::main+8C0h (00007FF7AF061B80h)
00007FF7AF061BF5  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF061BFC  test         rcx, rcx
00007FF7AF061BFF  jne          static void Fabled_Engine::main+959h (00007FF7AF061C19h)
00007FF7AF061C01  call         GetProcessHeap (00007FF7AF079AA2h)
00007FF7AF061C06  test         rax, rax
00007FF7AF061C09  je           static void Fabled_Engine::main+1478h (00007FF7AF062738h)
00007FF7AF061C0F  mov          rcx, rax
00007FF7AF061C12  mov          qword ptr [00007FF7AF0841C8h], rax
00007FF7AF061C19  mov          r8d, 1B00h
00007FF7AF061C1F  mov          edx, 8h
00007FF7AF061C24  call         HeapAlloc (00007FF7AF079AA8h)
00007FF7AF061C29  test         rax, rax
00007FF7AF061C2C  je           static void Fabled_Engine::main+1478h (00007FF7AF062738h)
00007FF7AF061C32  mov          rsi, rax
00007FF7AF061C35  xor          eax, eax
00007FF7AF061C37  movdqa       xmm0, xmmword ptr [__xmm@00000000000000110000000000000010 (00007FF7AF07D5D0h)]
00007FF7AF061C3F  movdqa       xmm1, xmmword ptr [__xmm@000000000000009900000000000000a9 (00007FF7AF07D5E0h)]
00007FF7AF061C47  mov          rcx, rsi
00007FF7AF061C4A  nop          word ptr [rax+rax*1], ax
00007FF7AF061C50  mov          qword ptr [rcx], rax
00007FF7AF061C53  movq         xmm2, rax
00007FF7AF061C58  pshufd       xmm2, xmm2, 44h
00007FF7AF061C5D  movdqa       xmm3, xmm2
00007FF7AF061C61  paddq        xmm3, xmm0
00007FF7AF061C65  movdqu       xmmword ptr [rcx+8h], xmm3
00007FF7AF061C6A  paddq        xmm2, xmm1
00007FF7AF061C6E  movdqu       xmmword ptr [rcx+1980h], xmm2
00007FF7AF061C76  lea          rdx, [rax+98h]
00007FF7AF061C7D  mov          qword ptr [rcx+1990h], rdx
00007FF7AF061C84  add          rcx, 18h
00007FF7AF061C88  add          rax, 1h
00007FF7AF061C8C  cmp          rax, 10h
00007FF7AF061C90  jne          static void Fabled_Engine::main+990h (00007FF7AF061C50h)
00007FF7AF061C92  mov          ecx, 210h
00007FF7AF061C97  mov          edx, 77h
00007FF7AF061C9C  nop          dword ptr [rax], eax
00007FF7AF061CA0  lea          rax, [rcx-1E0h]
00007FF7AF061CA7  cmp          rax, 360h
00007FF7AF061CAD  jae          static void Fabled_Engine::main+12F8h (00007FF7AF0625B8h)
00007FF7AF061CB3  lea          rbx, [rdx-67h]
00007FF7AF061CB7  mov          qword ptr [rsi+rcx*8-F00h], rbx
00007FF7AF061CBF  lea          rbp, [rcx-1DFh]
00007FF7AF061CC6  cmp          rbp, 360h
00007FF7AF061CCD  jae          static void Fabled_Engine::main+130Eh (00007FF7AF0625CEh)
00007FF7AF061CD3  lea          rbp, [rdx-55h]
00007FF7AF061CD7  mov          qword ptr [rsi+rcx*8-EF8h], rbp
00007FF7AF061CDF  cmp          rax, 35Eh
00007FF7AF061CE5  jae          static void Fabled_Engine::main+1324h (00007FF7AF0625E4h)
00007FF7AF061CEB  lea          rdi, [rdx-66h]
00007FF7AF061CEF  mov          qword ptr [rsi+rcx*8-EF0h], rdi
00007FF7AF061CF7  mov          qword ptr [rsi+rcx*8-EE8h], rbx
00007FF7AF061CFF  cmp          rax, 35Ch
00007FF7AF061D05  jae          static void Fabled_Engine::main+133Eh (00007FF7AF0625FEh)
00007FF7AF061D0B  lea          rax, [rdx-56h]
00007FF7AF061D0F  mov          qword ptr [rsi+rcx*8-EE0h], rax
00007FF7AF061D17  mov          qword ptr [rsi+rcx*8-ED8h], rbp
00007FF7AF061D1F  cmp          rcx, 360h
00007FF7AF061D26  jae          static void Fabled_Engine::main+1358h (00007FF7AF062618h)
00007FF7AF061D2C  lea          rbp, [rdx-12h]
00007FF7AF061D30  mov          qword ptr [rsi+rcx*8], rbp
00007FF7AF061D34  lea          rax, [rcx+1h]
00007FF7AF061D38  cmp          rax, 360h
00007FF7AF061D3E  jae          static void Fabled_Engine::main+136Bh (00007FF7AF06262Bh)
00007FF7AF061D44  mov          qword ptr [rsi+rcx*8+8h], rdx
00007FF7AF061D49  cmp          rcx, 35Eh
00007FF7AF061D50  jae          static void Fabled_Engine::main+1381h (00007FF7AF062641h)
00007FF7AF061D56  lea          rax, [rdx-11h]
00007FF7AF061D5A  mov          qword ptr [rsi+rcx*8+10h], rax
00007FF7AF061D5F  mov          qword ptr [rsi+rcx*8+18h], rbp
00007FF7AF061D64  cmp          rcx, 35Ch
00007FF7AF061D6B  jae          static void Fabled_Engine::main+1398h (00007FF7AF062658h)
00007FF7AF061D71  lea          rax, [rdx-1h]
00007FF7AF061D75  mov          qword ptr [rsi+rcx*8+20h], rax
00007FF7AF061D7A  mov          qword ptr [rsi+rcx*8+28h], rdx
00007FF7AF061D7F  add          rcx, 6h
00007FF7AF061D83  add          rdx, 1h
00007FF7AF061D87  cmp          rcx, 270h
00007FF7AF061D8E  jne          static void Fabled_Engine::main+9E0h (00007FF7AF061CA0h)
00007FF7AF061D94  mov          edx, 88h
00007FF7AF061D99  nop          dword ptr [rax], eax
00007FF7AF061DA0  lea          rax, [rcx-1E0h]
00007FF7AF061DA7  cmp          rax, 35Fh
00007FF7AF061DAD  ja           static void Fabled_Engine::main+12F8h (00007FF7AF0625B8h)
00007FF7AF061DB3  lea          rbx, [rdx-67h]
00007FF7AF061DB7  mov          qword ptr [rsi+rcx*8-F00h], rbx
00007FF7AF061DBF  lea          rbp, [rcx-1DFh]
00007FF7AF061DC6  cmp          rbp, 35Fh
00007FF7AF061DCD  ja           static void Fabled_Engine::main+130Eh (00007FF7AF0625CEh)
00007FF7AF061DD3  lea          rbp, [rdx-55h]
00007FF7AF061DD7  mov          qword ptr [rsi+rcx*8-EF8h], rbp
00007FF7AF061DDF  cmp          rax, 35Dh
00007FF7AF061DE5  ja           static void Fabled_Engine::main+1324h (00007FF7AF0625E4h)
00007FF7AF061DEB  lea          rdi, [rdx-66h]
00007FF7AF061DEF  mov          qword ptr [rsi+rcx*8-EF0h], rdi
00007FF7AF061DF7  mov          qword ptr [rsi+rcx*8-EE8h], rbx
00007FF7AF061DFF  cmp          rax, 35Bh
00007FF7AF061E05  ja           static void Fabled_Engine::main+133Eh (00007FF7AF0625FEh)
00007FF7AF061E0B  lea          rax, [rdx-56h]
00007FF7AF061E0F  mov          qword ptr [rsi+rcx*8-EE0h], rax
00007FF7AF061E17  mov          qword ptr [rsi+rcx*8-ED8h], rbp
00007FF7AF061E1F  cmp          rcx, 35Fh
00007FF7AF061E26  ja           static void Fabled_Engine::main+1358h (00007FF7AF062618h)
00007FF7AF061E2C  lea          rbp, [rdx-12h]
00007FF7AF061E30  mov          qword ptr [rsi+rcx*8], rbp
00007FF7AF061E34  lea          rax, [rcx+1h]
00007FF7AF061E38  cmp          rax, 35Fh
00007FF7AF061E3E  ja           static void Fabled_Engine::main+136Bh (00007FF7AF06262Bh)
00007FF7AF061E44  mov          qword ptr [rsi+rcx*8+8h], rdx
00007FF7AF061E49  cmp          rcx, 35Dh
00007FF7AF061E50  ja           static void Fabled_Engine::main+1381h (00007FF7AF062641h)
00007FF7AF061E56  lea          rax, [rdx-11h]
00007FF7AF061E5A  mov          qword ptr [rsi+rcx*8+10h], rax
00007FF7AF061E5F  mov          qword ptr [rsi+rcx*8+18h], rbp
00007FF7AF061E64  cmp          rcx, 35Bh
00007FF7AF061E6B  ja           static void Fabled_Engine::main+1398h (00007FF7AF062658h)
00007FF7AF061E71  lea          rax, [rdx-1h]
00007FF7AF061E75  mov          qword ptr [rsi+rcx*8+20h], rax
00007FF7AF061E7A  mov          qword ptr [rsi+rcx*8+28h], rdx
00007FF7AF061E7F  add          rcx, 6h
00007FF7AF061E83  add          rdx, 1h
00007FF7AF061E87  cmp          rdx, 98h
00007FF7AF061E8E  jne          static void Fabled_Engine::main+AE0h (00007FF7AF061DA0h)
00007FF7AF061E94  mov          edx, 99h
00007FF7AF061E99  nop          dword ptr [rax], eax
00007FF7AF061EA0  lea          rax, [rcx-1E0h]
00007FF7AF061EA7  cmp          rax, 35Fh
00007FF7AF061EAD  ja           static void Fabled_Engine::main+12F8h (00007FF7AF0625B8h)
00007FF7AF061EB3  lea          rbx, [rdx-67h]
00007FF7AF061EB7  mov          qword ptr [rsi+rcx*8-F00h], rbx
00007FF7AF061EBF  lea          rbp, [rcx-1DFh]
00007FF7AF061EC6  cmp          rbp, 35Fh
00007FF7AF061ECD  ja           static void Fabled_Engine::main+130Eh (00007FF7AF0625CEh)
00007FF7AF061ED3  lea          rbp, [rdx-55h]
00007FF7AF061ED7  mov          qword ptr [rsi+rcx*8-EF8h], rbp
00007FF7AF061EDF  cmp          rax, 35Dh
00007FF7AF061EE5  ja           static void Fabled_Engine::main+1324h (00007FF7AF0625E4h)
00007FF7AF061EEB  lea          rdi, [rdx-66h]
00007FF7AF061EEF  mov          qword ptr [rsi+rcx*8-EF0h], rdi
00007FF7AF061EF7  mov          qword ptr [rsi+rcx*8-EE8h], rbx
00007FF7AF061EFF  cmp          rax, 35Bh
00007FF7AF061F05  ja           static void Fabled_Engine::main+133Eh (00007FF7AF0625FEh)
00007FF7AF061F0B  lea          rax, [rdx-56h]
00007FF7AF061F0F  mov          qword ptr [rsi+rcx*8-EE0h], rax
00007FF7AF061F17  mov          qword ptr [rsi+rcx*8-ED8h], rbp
00007FF7AF061F1F  cmp          rcx, 35Fh
00007FF7AF061F26  ja           static void Fabled_Engine::main+1358h (00007FF7AF062618h)
00007FF7AF061F2C  lea          rbp, [rdx-12h]
00007FF7AF061F30  mov          qword ptr [rsi+rcx*8], rbp
00007FF7AF061F34  lea          rax, [rcx+1h]
00007FF7AF061F38  cmp          rax, 35Fh
00007FF7AF061F3E  ja           static void Fabled_Engine::main+136Bh (00007FF7AF06262Bh)
00007FF7AF061F44  mov          qword ptr [rsi+rcx*8+8h], rdx
00007FF7AF061F49  cmp          rcx, 35Dh
00007FF7AF061F50  ja           static void Fabled_Engine::main+1381h (00007FF7AF062641h)
00007FF7AF061F56  lea          rax, [rdx-11h]
00007FF7AF061F5A  mov          qword ptr [rsi+rcx*8+10h], rax
00007FF7AF061F5F  mov          qword ptr [rsi+rcx*8+18h], rbp
00007FF7AF061F64  cmp          rcx, 35Bh
00007FF7AF061F6B  ja           static void Fabled_Engine::main+1398h (00007FF7AF062658h)
00007FF7AF061F71  lea          rax, [rdx-1h]
00007FF7AF061F75  mov          qword ptr [rsi+rcx*8+20h], rax
00007FF7AF061F7A  mov          qword ptr [rsi+rcx*8+28h], rdx
00007FF7AF061F7F  add          rcx, 6h
00007FF7AF061F83  add          rdx, 1h
00007FF7AF061F87  cmp          rdx, A9h
00007FF7AF061F8E  jne          static void Fabled_Engine::main+BE0h (00007FF7AF061EA0h)
00007FF7AF061F94  mov          ecx, 150h
00007FF7AF061F99  mov          edx, 55h
00007FF7AF061F9E  nop
00007FF7AF061FA0  cmp          rcx, 35Fh
00007FF7AF061FA7  ja           static void Fabled_Engine::main+13AFh (00007FF7AF06266Fh)
00007FF7AF061FAD  lea          rbp, [rdx-12h]
00007FF7AF061FB1  mov          qword ptr [rsi+rcx*8], rbp
00007FF7AF061FB5  lea          rax, [rcx+1h]
00007FF7AF061FB9  cmp          rax, 35Fh
00007FF7AF061FBF  ja           static void Fabled_Engine::main+13C2h (00007FF7AF062682h)
00007FF7AF061FC5  mov          qword ptr [rsi+rcx*8+8h], rdx
00007FF7AF061FCA  cmp          rcx, 35Dh
00007FF7AF061FD1  ja           static void Fabled_Engine::main+13D8h (00007FF7AF062698h)
00007FF7AF061FD7  lea          rax, [rdx-11h]
00007FF7AF061FDB  mov          qword ptr [rsi+rcx*8+10h], rax
00007FF7AF061FE0  mov          qword ptr [rsi+rcx*8+18h], rbp
00007FF7AF061FE5  cmp          rcx, 35Bh
00007FF7AF061FEC  ja           static void Fabled_Engine::main+13EFh (00007FF7AF0626AFh)
00007FF7AF061FF2  lea          rax, [rdx-1h]
00007FF7AF061FF6  mov          qword ptr [rsi+rcx*8+20h], rax
00007FF7AF061FFB  mov          qword ptr [rsi+rcx*8+28h], rdx
00007FF7AF062000  add          rcx, 6h
00007FF7AF062004  add          rdx, 1h
00007FF7AF062008  cmp          rcx, 1B0h
00007FF7AF06200F  jne          static void Fabled_Engine::main+CE0h (00007FF7AF061FA0h)
00007FF7AF062011  mov          edx, 66h
00007FF7AF062016  nop          word ptr cs:[rax+rax*1], ax
00007FF7AF062020  cmp          rcx, 35Fh
00007FF7AF062027  ja           static void Fabled_Engine::main+13AFh (00007FF7AF06266Fh)
00007FF7AF06202D  lea          rbp, [rdx-12h]
00007FF7AF062031  mov          qword ptr [rsi+rcx*8], rbp
00007FF7AF062035  lea          rax, [rcx+1h]
00007FF7AF062039  cmp          rax, 35Fh
00007FF7AF06203F  ja           static void Fabled_Engine::main+13C2h (00007FF7AF062682h)
00007FF7AF062045  mov          qword ptr [rsi+rcx*8+8h], rdx
00007FF7AF06204A  cmp          rcx, 35Dh
00007FF7AF062051  ja           static void Fabled_Engine::main+13D8h (00007FF7AF062698h)
00007FF7AF062057  lea          rax, [rdx-11h]
00007FF7AF06205B  mov          qword ptr [rsi+rcx*8+10h], rax
00007FF7AF062060  mov          qword ptr [rsi+rcx*8+18h], rbp
00007FF7AF062065  cmp          rcx, 35Bh
00007FF7AF06206C  ja           static void Fabled_Engine::main+13EFh (00007FF7AF0626AFh)
00007FF7AF062072  lea          rax, [rdx-1h]
00007FF7AF062076  mov          qword ptr [rsi+rcx*8+20h], rax
00007FF7AF06207B  mov          qword ptr [rsi+rcx*8+28h], rdx
00007FF7AF062080  add          rcx, 6h
00007FF7AF062084  add          rdx, 1h
00007FF7AF062088  cmp          rdx, 76h
00007FF7AF06208C  jne          static void Fabled_Engine::main+D60h (00007FF7AF062020h)
00007FF7AF06208E  mov          qword ptr [rsp+C0h], 0h
00007FF7AF06209A  mov          dword ptr [rsp+C8h], 0h
00007FF7AF0620A5  mov          qword ptr [rsp+D0h], 0h
00007FF7AF0620B1  mov          dword ptr [rsp+D8h], 0h
00007FF7AF0620BC  pxor         xmm0, xmm0
00007FF7AF0620C0  movdqa       xmmword ptr [rsp+E0h], xmm0
00007FF7AF0620C9  movdqa       xmmword ptr [rsp+F0h], xmm0
00007FF7AF0620D2  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF0620D9  test         rcx, rcx
00007FF7AF0620DC  jne          static void Fabled_Engine::main+E36h (00007FF7AF0620F6h)
00007FF7AF0620DE  call         GetProcessHeap (00007FF7AF079AA2h)
00007FF7AF0620E3  test         rax, rax
00007FF7AF0620E6  je           static void Fabled_Engine::main+1489h (00007FF7AF062749h)
00007FF7AF0620EC  mov          rcx, rax
00007FF7AF0620EF  mov          qword ptr [00007FF7AF0841C8h], rax
00007FF7AF0620F6  mov          r8d, 2E40h
00007FF7AF0620FC  xor          edx, edx
00007FF7AF0620FE  call         HeapAlloc (00007FF7AF079AA8h)
00007FF7AF062103  test         rax, rax
00007FF7AF062106  je           static void Fabled_Engine::main+1489h (00007FF7AF062749h)
00007FF7AF06210C  mov          qword ptr [rsp+88h], rax
00007FF7AF062114  movaps       xmm0, xmmword ptr [__xmm@000000000000000000000000000000b9 (00007FF7AF07D5F0h)]
00007FF7AF06211B  movups       xmmword ptr [rsp+90h], xmm0
00007FF7AF062123  mov          rcx, qword ptr [rsp+C0h]
00007FF7AF06212B  mov          qword ptr [rsp+60h], rcx
00007FF7AF062130  mov          ecx, dword ptr [rsp+C8h]
00007FF7AF062137  mov          dword ptr [rsp+68h], ecx
00007FF7AF06213B  mov          rcx, qword ptr [rsp+D0h]
00007FF7AF062143  mov          qword ptr [rsp+78h], rcx
00007FF7AF062148  mov          ecx, dword ptr [rsp+D8h]
00007FF7AF06214F  mov          dword ptr [rsp+80h], ecx
00007FF7AF062156  movaps       xmm0, xmmword ptr [rsp+E0h]
00007FF7AF06215E  movaps       xmmword ptr [rsp+40h], xmm0
00007FF7AF062163  movaps       xmm0, xmmword ptr [rsp+F0h]
00007FF7AF06216B  movaps       xmmword ptr [rsp+30h], xmm0
00007FF7AF062170  xor          ecx, ecx
00007FF7AF062172  nop          word ptr cs:[rax+rax*1], ax
00007FF7AF06217C  nop          dword ptr [rax], eax
00007FF7AF062180  mov          edx, dword ptr [rsp+68h]
00007FF7AF062184  mov          dword ptr [rax+rcx*1+8h], edx
00007FF7AF062188  mov          rdx, qword ptr [rsp+60h]
00007FF7AF06218D  mov          qword ptr [rax+rcx*1], rdx
00007FF7AF062191  mov          qword ptr [rax+rcx*1+Ch], 0h
00007FF7AF06219A  mov          edx, dword ptr [rsp+80h]
00007FF7AF0621A1  mov          dword ptr [rax+rcx*1+1Ch], edx
00007FF7AF0621A5  mov          rdx, qword ptr [rsp+78h]
00007FF7AF0621AA  mov          qword ptr [rax+rcx*1+14h], rdx
00007FF7AF0621AF  movaps       xmm0, xmmword ptr [rsp+40h]
00007FF7AF0621B4  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF7AF0621B9  movaps       xmm0, xmmword ptr [rsp+30h]
00007FF7AF0621BE  movaps       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF7AF0621C3  add          rcx, 40h
00007FF7AF0621C7  cmp          rcx, 2E00h
00007FF7AF0621CE  jne          static void Fabled_Engine::main+EC0h (00007FF7AF062180h)
00007FF7AF0621D0  mov          edx, dword ptr [rsp+68h]
00007FF7AF0621D4  mov          dword ptr [rax+rcx*1+8h], edx
00007FF7AF0621D8  mov          rdx, qword ptr [rsp+60h]
00007FF7AF0621DD  mov          qword ptr [rax+rcx*1], rdx
00007FF7AF0621E1  mov          qword ptr [rax+rcx*1+Ch], 0h
00007FF7AF0621EA  mov          edx, dword ptr [rsp+80h]
00007FF7AF0621F1  mov          dword ptr [rax+rcx*1+1Ch], edx
00007FF7AF0621F5  mov          rdx, qword ptr [rsp+78h]
00007FF7AF0621FA  mov          qword ptr [rax+rcx*1+14h], rdx
00007FF7AF0621FF  movaps       xmm0, xmmword ptr [rsp+40h]
00007FF7AF062204  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF7AF062209  movaps       xmm0, xmmword ptr [rsp+30h]
00007FF7AF06220E  movaps       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF7AF062213  mov          qword ptr [rsp+98h], B9h
00007FF7AF06221F  mov          rax, qword ptr [rsp+88h]
00007FF7AF062227  add          rax, 20h
00007FF7AF06222B  xor          ecx, ecx
00007FF7AF06222D  xorps        xmm0, xmm0
00007FF7AF062230  mov          r8d, dword ptr [r13+rcx*2+8h]
00007FF7AF062235  mov          rbp, qword ptr [r13+rcx*2]
00007FF7AF06223A  mov          rbx, qword ptr [r15+rcx*1]
00007FF7AF06223E  mov          rdi, qword ptr [r12+rcx*2]
00007FF7AF062242  mov          edx, dword ptr [r12+rcx*2+8h]
00007FF7AF062247  mov          qword ptr [rax+rcx*8-20h], rbp
00007FF7AF06224C  mov          dword ptr [rax+rcx*8-18h], r8d
00007FF7AF062251  mov          qword ptr [rax+rcx*8-14h], rbx
00007FF7AF062256  mov          dword ptr [rax+rcx*8-4h], edx
00007FF7AF06225A  mov          qword ptr [rax+rcx*8-Ch], rdi
00007FF7AF06225F  movaps       xmmword ptr [rax+rcx*8], xmm0
00007FF7AF062263  movaps       xmmword ptr [rax+rcx*8+10h], xmm0
00007FF7AF062268  add          rcx, 8h
00007FF7AF06226C  cmp          rcx, 5C8h
00007FF7AF062273  jne          static void Fabled_Engine::main+F70h (00007FF7AF062230h)
00007FF7AF062275  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF06227C  test         rcx, rcx
00007FF7AF06227F  jne          static void Fabled_Engine::main+FD9h (00007FF7AF062299h)
00007FF7AF062281  call         GetProcessHeap (00007FF7AF079AA2h)
00007FF7AF062286  test         rax, rax
00007FF7AF062289  je           static void Fabled_Engine::main+1490h (00007FF7AF062750h)
00007FF7AF06228F  mov          rcx, rax
00007FF7AF062292  mov          qword ptr [00007FF7AF0841C8h], rax
00007FF7AF062299  mov          r8d, 40h
00007FF7AF06229F  xor          edx, edx
00007FF7AF0622A1  call         HeapAlloc (00007FF7AF079AA8h)
00007FF7AF0622A6  test         rax, rax
00007FF7AF0622A9  je           static void Fabled_Engine::main+1490h (00007FF7AF062750h)
00007FF7AF0622AF  mov          rbp, rax
00007FF7AF0622B2  mov          rax, qword ptr [rsp+98h]
00007FF7AF0622BA  mov          qword ptr [rbp+10h], rax
00007FF7AF0622BE  movups       xmm0, xmmword ptr [rsp+88h]
00007FF7AF0622C6  movups       xmmword ptr [rbp], xmm0
00007FF7AF0622CA  mov          qword ptr [rbp+20h], rsi
00007FF7AF0622CE  movaps       xmm0, xmmword ptr [__xmm@00000000000003600000000000000360 (00007FF7AF07D600h)]
00007FF7AF0622D5  movups       xmmword ptr [rbp+28h], xmm0
00007FF7AF0622D9  mov          dword ptr [rbp+18h], 0h
00007FF7AF0622E0  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF0622E7  xor          edx, edx
00007FF7AF0622E9  mov          r8, qword ptr [rsp+B8h]
00007FF7AF0622F1  call         HeapFree (00007FF7AF079AAEh)
00007FF7AF0622F6  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF0622FD  xor          edx, edx
00007FF7AF0622FF  mov          r8, qword ptr [rsp+70h]
00007FF7AF062304  call         HeapFree (00007FF7AF079AAEh)
00007FF7AF062309  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF062310  xor          edx, edx
00007FF7AF062312  mov          r8, r14
00007FF7AF062315  call         HeapFree (00007FF7AF079AAEh)
00007FF7AF06231A  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF062321  xor          edx, edx
00007FF7AF062323  mov          r8, r12
00007FF7AF062326  call         HeapFree (00007FF7AF079AAEh)
00007FF7AF06232B  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF062332  xor          edx, edx
00007FF7AF062334  mov          r8, r15
00007FF7AF062337  call         HeapFree (00007FF7AF079AAEh)
00007FF7AF06233C  mov          rcx, qword ptr [00007FF7AF0841C8h]
00007FF7AF062343  xor          edx, edx
00007FF7AF062345  mov          r8, r13
00007FF7AF062348  call         HeapFree (00007FF7AF079AAEh)
00007FF7AF06234D  lea          rax, [00007FF7AF080CB0h]
00007FF7AF062354  mov          qword ptr [rsp+E0h], rax
00007FF7AF06235C  mov          qword ptr [rsp+E8h], 6h

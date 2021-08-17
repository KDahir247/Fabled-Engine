# --------------- Capsule Dissassembly -------------------
00007FF60A521359  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A521360  test         rcx, rcx
00007FF60A521363  jne          static void Fabled_Engine::main+BDh (00007FF60A52137Dh)
00007FF60A521365  call         GetProcessHeap (00007FF60A539AA2h)
00007FF60A52136A  test         rax, rax
00007FF60A52136D  je           static void Fabled_Engine::main+1452h (00007FF60A522712h)
00007FF60A521373  mov          rcx, rax
00007FF60A521376  mov          qword ptr [00007FF60A5441C8h], rax
00007FF60A52137D  mov          r8d, B90h
00007FF60A521383  xor          edx, edx
00007FF60A521385  call         HeapAlloc (00007FF60A539AA8h)
00007FF60A52138A  test         rax, rax
00007FF60A52138D  je           static void Fabled_Engine::main+1452h (00007FF60A522712h)
00007FF60A521393  mov          r13, rax
00007FF60A521396  xor          eax, eax
00007FF60A521398  xorps        xmm0, xmm0
00007FF60A52139B  nop          dword ptr [rax+rax*1], eax
00007FF60A5213A0  movaps       xmmword ptr [r13+rax*1], xmm0
00007FF60A5213A6  movaps       xmmword ptr [r13+rax*1+10h], xmm0
00007FF60A5213AC  movaps       xmmword ptr [r13+rax*1+20h], xmm0
00007FF60A5213B2  movaps       xmmword ptr [r13+rax*1+30h], xmm0
00007FF60A5213B8  movaps       xmmword ptr [r13+rax*1+40h], xmm0
00007FF60A5213BE  movaps       xmmword ptr [r13+rax*1+50h], xmm0
00007FF60A5213C4  movaps       xmmword ptr [r13+rax*1+60h], xmm0
00007FF60A5213CA  movaps       xmmword ptr [r13+rax*1+70h], xmm0
00007FF60A5213D0  sub          rax, FFFFFFFFFFFFFF80h
00007FF60A5213D4  cmp          rax, B80h
00007FF60A5213DA  jne          static void Fabled_Engine::main+E0h (00007FF60A5213A0h)
00007FF60A5213DC  xorps        xmm0, xmm0
00007FF60A5213DF  movaps       xmmword ptr [r13+rax*1], xmm0
00007FF60A5213E5  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A5213EC  test         rcx, rcx
00007FF60A5213EF  jne          static void Fabled_Engine::main+149h (00007FF60A521409h)
00007FF60A5213F1  call         GetProcessHeap (00007FF60A539AA2h)
00007FF60A5213F6  test         rax, rax
00007FF60A5213F9  je           static void Fabled_Engine::main+1460h (00007FF60A522720h)
00007FF60A5213FF  mov          rcx, rax
00007FF60A521402  mov          qword ptr [00007FF60A5441C8h], rax
00007FF60A521409  mov          r8d, 5C8h
00007FF60A52140F  xor          edx, edx
00007FF60A521411  call         HeapAlloc (00007FF60A539AA8h)
00007FF60A521416  test         rax, rax
00007FF60A521419  je           static void Fabled_Engine::main+1460h (00007FF60A522720h)
00007FF60A52141F  mov          r15, rax
00007FF60A521422  xor          ecx, ecx
00007FF60A521424  xorps        xmm0, xmm0
00007FF60A521427  nop          word ptr [rax+rax*1], ax
00007FF60A521430  mov          rax, rcx
00007FF60A521433  movups       xmmword ptr [r15+rcx*8], xmm0
00007FF60A521438  movups       xmmword ptr [r15+rcx*8+10h], xmm0
00007FF60A52143E  movups       xmmword ptr [r15+rcx*8+20h], xmm0
00007FF60A521444  movups       xmmword ptr [r15+rcx*8+30h], xmm0
00007FF60A52144A  add          rcx, 8h
00007FF60A52144E  cmp          rcx, B8h
00007FF60A521455  jne          static void Fabled_Engine::main+170h (00007FF60A521430h)
00007FF60A521457  mov          qword ptr [r15+rax*8+40h], 0h
00007FF60A521460  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A521467  test         rcx, rcx
00007FF60A52146A  jne          static void Fabled_Engine::main+1C4h (00007FF60A521484h)
00007FF60A52146C  call         GetProcessHeap (00007FF60A539AA2h)
00007FF60A521471  test         rax, rax
00007FF60A521474  je           static void Fabled_Engine::main+1452h (00007FF60A522712h)
00007FF60A52147A  mov          rcx, rax
00007FF60A52147D  mov          qword ptr [00007FF60A5441C8h], rax
00007FF60A521484  mov          r8d, B90h
00007FF60A52148A  xor          edx, edx
00007FF60A52148C  call         HeapAlloc (00007FF60A539AA8h)
00007FF60A521491  test         rax, rax
00007FF60A521494  je           static void Fabled_Engine::main+1452h (00007FF60A522712h)
00007FF60A52149A  mov          r12, rax
00007FF60A52149D  xor          eax, eax
00007FF60A52149F  xorps        xmm0, xmm0
00007FF60A5214A2  nop          word ptr cs:[rax+rax*1], ax
00007FF60A5214AC  nop          dword ptr [rax], eax
00007FF60A5214B0  movaps       xmmword ptr [r12+rax*1], xmm0
00007FF60A5214B5  movaps       xmmword ptr [r12+rax*1+10h], xmm0
00007FF60A5214BB  movaps       xmmword ptr [r12+rax*1+20h], xmm0
00007FF60A5214C1  movaps       xmmword ptr [r12+rax*1+30h], xmm0
00007FF60A5214C7  movaps       xmmword ptr [r12+rax*1+40h], xmm0
00007FF60A5214CD  movaps       xmmword ptr [r12+rax*1+50h], xmm0
00007FF60A5214D3  movaps       xmmword ptr [r12+rax*1+60h], xmm0
00007FF60A5214D9  movaps       xmmword ptr [r12+rax*1+70h], xmm0
00007FF60A5214DF  sub          rax, FFFFFFFFFFFFFF80h
00007FF60A5214E3  cmp          rax, B80h
00007FF60A5214E9  jne          static void Fabled_Engine::main+1F0h (00007FF60A5214B0h)
00007FF60A5214EB  xorps        xmm0, xmm0
00007FF60A5214EE  movaps       xmmword ptr [r12+rax*1], xmm0
00007FF60A5214F3  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A5214FA  test         rcx, rcx
00007FF60A5214FD  jne          static void Fabled_Engine::main+257h (00007FF60A521517h)
00007FF60A5214FF  call         GetProcessHeap (00007FF60A539AA2h)
00007FF60A521504  test         rax, rax
00007FF60A521507  je           static void Fabled_Engine::main+1459h (00007FF60A522719h)
00007FF60A52150D  mov          rcx, rax
00007FF60A521510  mov          qword ptr [00007FF60A5441C8h], rax
00007FF60A521517  mov          r8d, 80h
00007FF60A52151D  xor          edx, edx
00007FF60A52151F  call         HeapAlloc (00007FF60A539AA8h)
00007FF60A521524  test         rax, rax
00007FF60A521527  je           static void Fabled_Engine::main+1459h (00007FF60A522719h)
00007FF60A52152D  mov          r14, rax
00007FF60A521530  xorps        xmm0, xmm0
00007FF60A521533  movups       xmmword ptr [rax], xmm0
00007FF60A521536  movups       xmmword ptr [rax+10h], xmm0
00007FF60A52153A  movups       xmmword ptr [rax+20h], xmm0
00007FF60A52153E  movups       xmmword ptr [rax+30h], xmm0
00007FF60A521542  movups       xmmword ptr [rax+40h], xmm0
00007FF60A521546  movups       xmmword ptr [rax+50h], xmm0
00007FF60A52154A  movups       xmmword ptr [rax+60h], xmm0
00007FF60A52154E  movups       xmmword ptr [rax+70h], xmm0
00007FF60A521552  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A521559  test         rcx, rcx
00007FF60A52155C  jne          static void Fabled_Engine::main+2B6h (00007FF60A521576h)
00007FF60A52155E  call         GetProcessHeap (00007FF60A539AA2h)
00007FF60A521563  test         rax, rax
00007FF60A521566  je           static void Fabled_Engine::main+1459h (00007FF60A522719h)
00007FF60A52156C  mov          rcx, rax
00007FF60A52156F  mov          qword ptr [00007FF60A5441C8h], rax
00007FF60A521576  mov          r8d, 80h
00007FF60A52157C  xor          edx, edx
00007FF60A52157E  call         HeapAlloc (00007FF60A539AA8h)
00007FF60A521583  test         rax, rax
00007FF60A521586  je           static void Fabled_Engine::main+1459h (00007FF60A522719h)
00007FF60A52158C  mov          rsi, rax
00007FF60A52158F  xorps        xmm0, xmm0
00007FF60A521592  movups       xmmword ptr [rax], xmm0
00007FF60A521595  movups       xmmword ptr [rax+10h], xmm0
00007FF60A521599  movups       xmmword ptr [rax+20h], xmm0
00007FF60A52159D  movups       xmmword ptr [rax+30h], xmm0
00007FF60A5215A1  movups       xmmword ptr [rax+40h], xmm0
00007FF60A5215A5  movups       xmmword ptr [rax+50h], xmm0
00007FF60A5215A9  movups       xmmword ptr [rax+60h], xmm0
00007FF60A5215AD  movups       xmmword ptr [rax+70h], xmm0
00007FF60A5215B1  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A5215B8  test         rcx, rcx
00007FF60A5215BB  jne          static void Fabled_Engine::main+315h (00007FF60A5215D5h)
00007FF60A5215BD  call         GetProcessHeap (00007FF60A539AA2h)
00007FF60A5215C2  test         rax, rax
00007FF60A5215C5  je           static void Fabled_Engine::main+1467h (00007FF60A522727h)
00007FF60A5215CB  mov          rcx, rax
00007FF60A5215CE  mov          qword ptr [00007FF60A5441C8h], rax
00007FF60A5215D5  mov          r8d, 44h
00007FF60A5215DB  mov          edx, 8h
00007FF60A5215E0  call         HeapAlloc (00007FF60A539AA8h)
00007FF60A5215E5  mov          qword ptr [rsp+B8h], rax
00007FF60A5215ED  test         rax, rax
00007FF60A5215F0  je           static void Fabled_Engine::main+1467h (00007FF60A522727h)
00007FF60A5215F6  mov          qword ptr [rsp+70h], rsi
00007FF60A5215FB  xor          esi, esi
00007FF60A5215FD  movss        xmm10, dword ptr [__real@3f000000 (00007FF60A53D530h)]
00007FF60A521606  movss        xmm8, dword ptr [__real@bd800000 (00007FF60A53D534h)]
00007FF60A52160F  movss        xmm9, dword ptr [__real@3f800000 (00007FF60A53D538h)]
00007FF60A521618  movss        xmm11, dword ptr [__real@3ec90fdb (00007FF60A53D53Ch)]
00007FF60A521621  movaps       xmm12, xmmword ptr [__xmm@00000000000000003f80000000000000 (00007FF60A53D540h)]
00007FF60A521629  movaps       xmm13, xmmword ptr [__xmm@0000000000000000bf80000000000000 (00007FF60A53D550h)]
00007FF60A521631  xor          ebp, ebp
00007FF60A521633  jmp          static void Fabled_Engine::main+437h (00007FF60A5216F7h)
00007FF60A521638  nop          dword ptr [rax+rax*1], eax
00007FF60A521640  xorps        xmm7, xmm7
00007FF60A521643  cvtsi2ss     xmm7, rbp
00007FF60A521648  add          rbp, 1h
00007FF60A52164C  movaps       xmm6, xmm7
00007FF60A52164F  addss        xmm6, xmm10
00007FF60A521654  mulss        xmm6, xmm8
00007FF60A521659  addss        xmm6, xmm9
00007FF60A52165E  mulss        xmm7, xmm11
00007FF60A521663  movaps       xmm0, xmm7
00007FF60A521666  call         sinf (00007FF60A53ABC9h)
00007FF60A52166B  movaps       xmm14, xmm0
00007FF60A52166F  movaps       xmm0, xmm7
00007FF60A521672  call         cosf (00007FF60A53ABCFh)
00007FF60A521677  movss        dword ptr [r14+rsi*1], xmm0
00007FF60A52167D  movss        dword ptr [r14+rsi*1+4h], xmm14
00007FF60A521684  mulss        xmm14, xmm10
00007FF60A521689  mulss        xmm0, xmm10
00007FF60A52168E  mov          rax, qword ptr [rsp+70h]
00007FF60A521693  movss        dword ptr [rax+rsi*1], xmm0
00007FF60A521698  movss        dword ptr [rax+rsi*1+4h], xmm14
00007FF60A52169F  movaps       xmmword ptr [r13+rsi*2], xmm12
00007FF60A5216A5  movss        dword ptr [r15+rsi*1], xmm6
00007FF60A5216AB  mov          dword ptr [r15+rsi*1+4h], 3F800000h
00007FF60A5216B4  movaps       xmmword ptr [r12+rsi*2], xmm12
00007FF60A5216B9  xorps        xmm0, xmm0
00007FF60A5216BC  subps        xmm0, xmmword ptr [r13+rsi*2]
00007FF60A5216C2  movaps       xmmword ptr [r13+rsi*2+A90h], xmm0
00007FF60A5216CB  movss        dword ptr [r15+rsi*1+548h], xmm6
00007FF60A5216D5  mov          dword ptr [r15+rsi*1+54Ch], 0h
00007FF60A5216E1  movaps       xmmword ptr [r12+rsi*2+A90h], xmm13
00007FF60A5216EA  add          rsi, 8h
00007FF60A5216EE  cmp          rsi, 80h
00007FF60A5216F5  je           static void Fabled_Engine::main+45Fh (00007FF60A52171Fh)
00007FF60A5216F7  test         rbp, rbp
00007FF60A5216FA  jns          static void Fabled_Engine::main+380h (00007FF60A521640h)
00007FF60A521700  mov          rax, rbp
00007FF60A521703  shr          rax, 1h
00007FF60A521706  mov          ecx, ebp
00007FF60A521708  and          ecx, 1h
00007FF60A52170B  or           rcx, rax
00007FF60A52170E  xorps        xmm7, xmm7
00007FF60A521711  cvtsi2ss     xmm7, rcx
00007FF60A521716  addss        xmm7, xmm7
00007FF60A52171A  jmp          static void Fabled_Engine::main+388h (00007FF60A521648h)
00007FF60A52171F  xor          eax, eax
00007FF60A521721  movaps       xmm11, xmmword ptr [__xmm@80000000800000008000000080000000 (00007FF60A53D560h)]
00007FF60A521729  movss        xmm1, dword ptr [__real@3f000000 (00007FF60A53D530h)]
00007FF60A521731  movss        xmm2, dword ptr [__real@bf000000 (00007FF60A53D570h)]
00007FF60A521739  xor          ecx, ecx
00007FF60A52173B  mov          rbx, qword ptr [rsp+70h]
00007FF60A521740  mov          rdi, qword ptr [rsp+B8h]
00007FF60A521748  jmp          static void Fabled_Engine::main+54Dh (00007FF60A52180Dh)
00007FF60A52174D  nop          dword ptr [rax], eax
00007FF60A521750  xorps        xmm0, xmm0
00007FF60A521753  cvtsi2ss     xmm0, rcx
00007FF60A521758  lea          edx, [rcx*8]
00007FF60A52175F  add          rcx, 1h
00007FF60A521763  mulss        xmm0, xmm8
00007FF60A521768  addss        xmm0, xmm9
00007FF60A52176D  movss        dword ptr [rdi+rax*1], xmm0
00007FF60A521772  and          edx, 78h
00007FF60A521775  movss        xmm3, dword ptr [rbx+rdx*1]
00007FF60A52177A  movss        xmm4, dword ptr [rbx+rdx*1+4h]
00007FF60A521780  xorps        xmm4, xmm11
00007FF60A521784  movaps       xmm5, xmm3
00007FF60A521787  unpcklps     xmm5, xmm1
00007FF60A52178A  shufps       xmm5, xmm4, 4h
00007FF60A52178E  movss        xmm6, dword ptr [r14+rdx*1]
00007FF60A521794  movss        xmm7, dword ptr [r14+rdx*1+4h]
00007FF60A52179B  movaps       xmmword ptr [r13+rax*4+430h], xmm5
00007FF60A5217A4  movss        dword ptr [r15+rax*2+218h], xmm0
00007FF60A5217AE  mov          dword ptr [r15+rax*2+21Ch], 3F400000h
00007FF60A5217BA  xorps        xmm7, xmm11
00007FF60A5217BE  shufps       xmm6, xmm7, 4h
00007FF60A5217C2  movaps       xmmword ptr [r12+rax*4+430h], xmm6
00007FF60A5217CB  unpcklps     xmm3, xmm2
00007FF60A5217CE  shufps       xmm3, xmm4, 4h
00007FF60A5217D2  movaps       xmmword ptr [r13+rax*4+650h], xmm3
00007FF60A5217DB  movss        dword ptr [r15+rax*2+328h], xmm0
00007FF60A5217E5  mov          dword ptr [r15+rax*2+32Ch], 3E800000h
00007FF60A5217F1  movaps       xmm0, xmmword ptr [r12+rax*4+430h]
00007FF60A5217FA  movaps       xmmword ptr [r12+rax*4+650h], xmm0
00007FF60A521803  add          rax, 4h
00007FF60A521807  cmp          rax, 44h
00007FF60A52180B  je           static void Fabled_Engine::main+575h (00007FF60A521835h)
00007FF60A52180D  test         rcx, rcx
00007FF60A521810  jns          static void Fabled_Engine::main+490h (00007FF60A521750h)
00007FF60A521816  mov          rdx, rcx
00007FF60A521819  shr          rdx, 1h
00007FF60A52181C  mov          ebp, ecx
00007FF60A52181E  and          ebp, 1h
00007FF60A521821  or           rbp, rdx
00007FF60A521824  xorps        xmm0, xmm0
00007FF60A521827  cvtsi2ss     xmm0, rbp
00007FF60A52182C  addss        xmm0, xmm0
00007FF60A521830  jmp          static void Fabled_Engine::main+498h (00007FF60A521758h)
00007FF60A521835  xor          eax, eax
00007FF60A521837  movss        xmm10, dword ptr [__real@be43ef16 (00007FF60A53D574h)]
00007FF60A521840  movss        xmm12, dword ptr [__real@3e43ef16 (00007FF60A53D578h)]
00007FF60A521849  movss        xmm13, dword ptr [__real@3f7641af (00007FF60A53D57Ch)]
00007FF60A521852  movss        xmm2, dword ptr [__real@bec3ef16 (00007FF60A53D580h)]
00007FF60A52185A  movss        xmm14, dword ptr [__real@3ec3ef16 (00007FF60A53D584h)]
00007FF60A521863  movss        xmm8, dword ptr [__real@3f6c835e (00007FF60A53D588h)]
00007FF60A52186C  movss        xmm1, dword ptr [__real@beec835e (00007FF60A53D58Ch)]
00007FF60A521874  movss        xmm15, dword ptr [__real@bf30fbc6 (00007FF60A53D594h)]
00007FF60A52187D  movss        xmm9, dword ptr [__real@bf6c835e (00007FF60A53D598h)]
00007FF60A521886  xor          edx, edx
00007FF60A521888  nop          dword ptr [rax+rax*1], eax
00007FF60A521890  lea          rcx, [rdx+1h]
00007FF60A521894  movss        xmm7, dword ptr [rdi+rax*1]
00007FF60A521899  and          edx, Fh
00007FF60A52189C  movss        xmm5, dword ptr [r14+rdx*8]
00007FF60A5218A2  movss        xmm3, dword ptr [r14+rdx*8+4h]
00007FF60A5218A9  movaps       xmm6, xmm3
00007FF60A5218AC  mulss        xmm6, xmm10
00007FF60A5218B1  movaps       xmm0, xmm5
00007FF60A5218B4  mulss        xmm0, xmm12
00007FF60A5218B9  unpcklps     xmm0, xmm13
00007FF60A5218BD  shufps       xmm0, xmm6, 4h
00007FF60A5218C1  movaps       xmmword ptr [r13+rax*4+100h], xmm0
00007FF60A5218CA  movss        dword ptr [r15+rax*2+80h], xmm7
00007FF60A5218D4  mov          dword ptr [r15+rax*2+84h], 3F700000h
00007FF60A5218E0  movaps       xmm0, xmm3
00007FF60A5218E3  mulss        xmm0, xmm2
00007FF60A5218E7  movaps       xmm6, xmm5
00007FF60A5218EA  mulss        xmm6, xmm14
00007FF60A5218EF  unpcklps     xmm6, xmm8
00007FF60A5218F3  shufps       xmm6, xmm0, 4h
00007FF60A5218F7  movaps       xmmword ptr [r12+rax*4+100h], xmm6
00007FF60A521900  movaps       xmm0, xmm3
00007FF60A521903  mulss        xmm0, xmm1
00007FF60A521907  movaps       xmm6, xmm5
00007FF60A52190A  mulss        xmm6, dword ptr [__real@3eec835e (00007FF60A53D590h)]
00007FF60A521912  unpcklps     xmm6, xmm15
00007FF60A521916  shufps       xmm6, xmm0, 4h
00007FF60A52191A  movaps       xmmword ptr [r13+rax*4+760h], xmm6
00007FF60A521923  movss        dword ptr [r15+rax*2+3B0h], xmm7
00007FF60A52192D  mov          dword ptr [r15+rax*2+3B4h], 3E400000h
00007FF60A521939  mulss        xmm3, xmm9
00007FF60A52193E  mulss        xmm5, xmm8
00007FF60A521943  unpcklps     xmm5, xmm2
00007FF60A521946  shufps       xmm5, xmm3, 4h
00007FF60A52194A  movaps       xmmword ptr [r12+rax*4+760h], xmm5
00007FF60A521953  add          rax, 4h
00007FF60A521957  mov          rdx, rcx
00007FF60A52195A  cmp          rax, 44h
00007FF60A52195E  jne          static void Fabled_Engine::main+5D0h (00007FF60A521890h)
00007FF60A521964  movaps       xmm15, xmm1
00007FF60A521968  xor          eax, eax
00007FF60A52196A  movss        xmm10, dword ptr [__real@beb504f3 (00007FF60A53D59Ch)]
00007FF60A521973  movss        xmm12, dword ptr [__real@3eb504f3 (00007FF60A53D5A0h)]
00007FF60A52197C  movss        xmm13, dword ptr [__real@3f5a827a (00007FF60A53D5A4h)]
00007FF60A521985  movss        xmm5, dword ptr [__real@bf3504f3 (00007FF60A53D5A8h)]
00007FF60A52198D  movss        xmm6, dword ptr [__real@3f3504f3 (00007FF60A53D5ACh)]
00007FF60A521995  movss        xmm14, dword ptr [__real@bf5a827a (00007FF60A53D5B0h)]
00007FF60A52199E  xor          edx, edx
00007FF60A5219A0  lea          rcx, [rdx+1h]
00007FF60A5219A4  movss        xmm2, dword ptr [rdi+rax*1]
00007FF60A5219A9  and          edx, Fh
00007FF60A5219AC  movss        xmm4, dword ptr [r14+rdx*8]
00007FF60A5219B2  movss        xmm3, dword ptr [r14+rdx*8+4h]
00007FF60A5219B9  movaps       xmm7, xmm3
00007FF60A5219BC  mulss        xmm7, xmm10
00007FF60A5219C1  movaps       xmm1, xmm4
00007FF60A5219C4  mulss        xmm1, xmm12
00007FF60A5219C9  movaps       xmm0, xmm1
00007FF60A5219CC  unpcklps     xmm0, xmm13
00007FF60A5219D0  shufps       xmm0, xmm7, 4h
00007FF60A5219D4  movaps       xmmword ptr [r13+rax*4+210h], xmm0
00007FF60A5219DD  movss        dword ptr [r15+rax*2+108h], xmm2
00007FF60A5219E7  mov          dword ptr [r15+rax*2+10Ch], 3F600000h
00007FF60A5219F3  mulss        xmm3, xmm5
00007FF60A5219F7  mulss        xmm4, xmm6
00007FF60A5219FB  movaps       xmm0, xmm4
00007FF60A5219FE  unpcklps     xmm0, xmm6
00007FF60A521A01  shufps       xmm0, xmm3, 4h
00007FF60A521A05  movaps       xmmword ptr [r12+rax*4+210h], xmm0
00007FF60A521A0E  unpcklps     xmm1, xmm14
00007FF60A521A12  shufps       xmm1, xmm7, 4h
00007FF60A521A16  movaps       xmmword ptr [r13+rax*4+870h], xmm1
00007FF60A521A1F  movss        dword ptr [r15+rax*2+438h], xmm2
00007FF60A521A29  mov          dword ptr [r15+rax*2+43Ch], 3E000000h
00007FF60A521A35  unpcklps     xmm4, xmm5
00007FF60A521A38  shufps       xmm4, xmm3, 4h
00007FF60A521A3C  movaps       xmmword ptr [r12+rax*4+870h], xmm4
00007FF60A521A45  add          rax, 4h
00007FF60A521A49  mov          rdx, rcx
00007FF60A521A4C  cmp          rax, 44h
00007FF60A521A50  jne          static void Fabled_Engine::main+6E0h (00007FF60A5219A0h)
00007FF60A521A56  xor          eax, eax
00007FF60A521A58  movss        xmm10, dword ptr [__real@3f30fbc5 (00007FF60A53D5B4h)]
00007FF60A521A61  movss        xmm3, dword ptr [__real@3ec3ef15 (00007FF60A53D5B8h)]
00007FF60A521A69  movss        xmm12, dword ptr [__real@be43ef15 (00007FF60A53D5BCh)]
00007FF60A521A72  movss        xmm13, dword ptr [__real@3e43ef15 (00007FF60A53D5C0h)]
00007FF60A521A7B  movss        xmm14, dword ptr [__real@bf7641af (00007FF60A53D5C4h)]
00007FF60A521A84  movss        xmm0, dword ptr [__real@bec3ef15 (00007FF60A53D5C8h)]
00007FF60A521A8C  xor          edx, edx
00007FF60A521A8E  movss        xmm7, dword ptr [__real@3eec835e (00007FF60A53D590h)]
00007FF60A521A96  nop          word ptr cs:[rax+rax*1], ax
00007FF60A521AA0  lea          rcx, [rdx+1h]
00007FF60A521AA4  movss        xmm6, dword ptr [rdi+rax*1]
00007FF60A521AA9  and          edx, Fh
00007FF60A521AAC  movss        xmm2, dword ptr [r14+rdx*8]
00007FF60A521AB2  movss        xmm5, dword ptr [r14+rdx*8+4h]
00007FF60A521AB9  movaps       xmm1, xmm5
00007FF60A521ABC  mulss        xmm1, xmm15
00007FF60A521AC1  movaps       xmm4, xmm2
00007FF60A521AC4  mulss        xmm4, xmm7
00007FF60A521AC8  unpcklps     xmm4, xmm10
00007FF60A521ACC  shufps       xmm4, xmm1, 4h
00007FF60A521AD0  movaps       xmmword ptr [r13+rax*4+320h], xmm4
00007FF60A521AD9  movss        dword ptr [r15+rax*2+190h], xmm6
00007FF60A521AE3  mov          dword ptr [r15+rax*2+194h], 3F500000h
00007FF60A521AEF  movaps       xmm1, xmm5
00007FF60A521AF2  mulss        xmm1, xmm9
00007FF60A521AF7  movaps       xmm4, xmm2
00007FF60A521AFA  mulss        xmm4, xmm8
00007FF60A521AFF  unpcklps     xmm4, xmm3
00007FF60A521B02  shufps       xmm4, xmm1, 4h
00007FF60A521B06  movaps       xmmword ptr [r12+rax*4+320h], xmm4
00007FF60A521B0F  movaps       xmm1, xmm5
00007FF60A521B12  mulss        xmm1, xmm12
00007FF60A521B17  movaps       xmm4, xmm2
00007FF60A521B1A  mulss        xmm4, xmm13
00007FF60A521B1F  unpcklps     xmm4, xmm14
00007FF60A521B23  shufps       xmm4, xmm1, 4h
00007FF60A521B27  movaps       xmmword ptr [r13+rax*4+980h], xmm4
00007FF60A521B30  movss        dword ptr [r15+rax*2+4C0h], xmm6
00007FF60A521B3A  mov          dword ptr [r15+rax*2+4C4h], 3D800000h
00007FF60A521B46  mulss        xmm5, xmm0
00007FF60A521B4A  mulss        xmm2, xmm3
00007FF60A521B4E  unpcklps     xmm2, xmm9
00007FF60A521B52  shufps       xmm2, xmm5, 4h
00007FF60A521B56  movaps       xmmword ptr [r12+rax*4+980h], xmm2
00007FF60A521B5F  add          rax, 4h
00007FF60A521B63  mov          rdx, rcx
00007FF60A521B66  cmp          rax, 44h
00007FF60A521B6A  jne          static void Fabled_Engine::main+7E0h (00007FF60A521AA0h)
00007FF60A521B70  mov          ecx, 54h
00007FF60A521B75  xor          eax, eax
00007FF60A521B77  nop          word ptr [rax+rax*1], ax
00007FF60A521B80  cmp          rax, 194h
00007FF60A521B86  je           static void Fabled_Engine::main+143Fh (00007FF60A5226FFh)
00007FF60A521B8C  lea          edx, [rcx-54h]
00007FF60A521B8F  and          edx, Fh
00007FF60A521B92  movss        xmm0, dword ptr [rbx+rdx*8+4h]
00007FF60A521B98  xorps        xmm0, xmm11
00007FF60A521B9C  movss        xmm1, dword ptr [rdi+rax*1]
00007FF60A521BA1  movss        xmm2, dword ptr [rbx+rdx*8]
00007FF60A521BA6  shufps       xmm2, xmm0, 4h
00007FF60A521BAA  movss        xmm0, dword ptr [r14+rdx*8]
00007FF60A521BB0  movss        xmm3, dword ptr [r14+rdx*8+4h]
00007FF60A521BB7  movaps       xmmword ptr [r13+rax*4+540h], xmm2
00007FF60A521BC0  movss        dword ptr [r15+rax*2+2A0h], xmm1
00007FF60A521BCA  mov          dword ptr [r15+rax*2+2A4h], 3F000000h
00007FF60A521BD6  xorps        xmm3, xmm11
00007FF60A521BDA  shufps       xmm0, xmm3, 4h
00007FF60A521BDE  movaps       xmmword ptr [r12+rax*4+540h], xmm0
00007FF60A521BE7  add          rcx, 1h
00007FF60A521BEB  add          rax, 4h
00007FF60A521BEF  cmp          rax, 44h
00007FF60A521BF3  jne          static void Fabled_Engine::main+8C0h (00007FF60A521B80h)
00007FF60A521BF5  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A521BFC  test         rcx, rcx
00007FF60A521BFF  jne          static void Fabled_Engine::main+959h (00007FF60A521C19h)
00007FF60A521C01  call         GetProcessHeap (00007FF60A539AA2h)
00007FF60A521C06  test         rax, rax
00007FF60A521C09  je           static void Fabled_Engine::main+1478h (00007FF60A522738h)
00007FF60A521C0F  mov          rcx, rax
00007FF60A521C12  mov          qword ptr [00007FF60A5441C8h], rax
00007FF60A521C19  mov          r8d, 1B00h
00007FF60A521C1F  mov          edx, 8h
00007FF60A521C24  call         HeapAlloc (00007FF60A539AA8h)
00007FF60A521C29  test         rax, rax
00007FF60A521C2C  je           static void Fabled_Engine::main+1478h (00007FF60A522738h)
00007FF60A521C32  mov          rsi, rax
00007FF60A521C35  xor          eax, eax
00007FF60A521C37  movdqa       xmm0, xmmword ptr [__xmm@00000000000000110000000000000010 (00007FF60A53D5D0h)]
00007FF60A521C3F  movdqa       xmm1, xmmword ptr [__xmm@000000000000009900000000000000a9 (00007FF60A53D5E0h)]
00007FF60A521C47  mov          rcx, rsi
00007FF60A521C4A  nop          word ptr [rax+rax*1], ax
00007FF60A521C50  mov          qword ptr [rcx], rax
00007FF60A521C53  movq         xmm2, rax
00007FF60A521C58  pshufd       xmm2, xmm2, 44h
00007FF60A521C5D  movdqa       xmm3, xmm2
00007FF60A521C61  paddq        xmm3, xmm0
00007FF60A521C65  movdqu       xmmword ptr [rcx+8h], xmm3
00007FF60A521C6A  paddq        xmm2, xmm1
00007FF60A521C6E  movdqu       xmmword ptr [rcx+1980h], xmm2
00007FF60A521C76  lea          rdx, [rax+98h]
00007FF60A521C7D  mov          qword ptr [rcx+1990h], rdx
00007FF60A521C84  add          rcx, 18h
00007FF60A521C88  add          rax, 1h
00007FF60A521C8C  cmp          rax, 10h
00007FF60A521C90  jne          static void Fabled_Engine::main+990h (00007FF60A521C50h)
00007FF60A521C92  mov          ecx, 210h
00007FF60A521C97  mov          edx, 77h
00007FF60A521C9C  nop          dword ptr [rax], eax
00007FF60A521CA0  lea          rax, [rcx-1E0h]
00007FF60A521CA7  cmp          rax, 360h
00007FF60A521CAD  jae          static void Fabled_Engine::main+12F8h (00007FF60A5225B8h)
00007FF60A521CB3  lea          rbx, [rdx-67h]
00007FF60A521CB7  mov          qword ptr [rsi+rcx*8-F00h], rbx
00007FF60A521CBF  lea          rbp, [rcx-1DFh]
00007FF60A521CC6  cmp          rbp, 360h
00007FF60A521CCD  jae          static void Fabled_Engine::main+130Eh (00007FF60A5225CEh)
00007FF60A521CD3  lea          rbp, [rdx-55h]
00007FF60A521CD7  mov          qword ptr [rsi+rcx*8-EF8h], rbp
00007FF60A521CDF  cmp          rax, 35Eh
00007FF60A521CE5  jae          static void Fabled_Engine::main+1324h (00007FF60A5225E4h)
00007FF60A521CEB  lea          rdi, [rdx-66h]
00007FF60A521CEF  mov          qword ptr [rsi+rcx*8-EF0h], rdi
00007FF60A521CF7  mov          qword ptr [rsi+rcx*8-EE8h], rbx
00007FF60A521CFF  cmp          rax, 35Ch
00007FF60A521D05  jae          static void Fabled_Engine::main+133Eh (00007FF60A5225FEh)
00007FF60A521D0B  lea          rax, [rdx-56h]
00007FF60A521D0F  mov          qword ptr [rsi+rcx*8-EE0h], rax
00007FF60A521D17  mov          qword ptr [rsi+rcx*8-ED8h], rbp
00007FF60A521D1F  cmp          rcx, 360h
00007FF60A521D26  jae          static void Fabled_Engine::main+1358h (00007FF60A522618h)
00007FF60A521D2C  lea          rbp, [rdx-12h]
00007FF60A521D30  mov          qword ptr [rsi+rcx*8], rbp
00007FF60A521D34  lea          rax, [rcx+1h]
00007FF60A521D38  cmp          rax, 360h
00007FF60A521D3E  jae          static void Fabled_Engine::main+136Bh (00007FF60A52262Bh)
00007FF60A521D44  mov          qword ptr [rsi+rcx*8+8h], rdx
00007FF60A521D49  cmp          rcx, 35Eh
00007FF60A521D50  jae          static void Fabled_Engine::main+1381h (00007FF60A522641h)
00007FF60A521D56  lea          rax, [rdx-11h]
00007FF60A521D5A  mov          qword ptr [rsi+rcx*8+10h], rax
00007FF60A521D5F  mov          qword ptr [rsi+rcx*8+18h], rbp
00007FF60A521D64  cmp          rcx, 35Ch
00007FF60A521D6B  jae          static void Fabled_Engine::main+1398h (00007FF60A522658h)
00007FF60A521D71  lea          rax, [rdx-1h]
00007FF60A521D75  mov          qword ptr [rsi+rcx*8+20h], rax
00007FF60A521D7A  mov          qword ptr [rsi+rcx*8+28h], rdx
00007FF60A521D7F  add          rcx, 6h
00007FF60A521D83  add          rdx, 1h
00007FF60A521D87  cmp          rcx, 270h
00007FF60A521D8E  jne          static void Fabled_Engine::main+9E0h (00007FF60A521CA0h)
00007FF60A521D94  mov          edx, 88h
00007FF60A521D99  nop          dword ptr [rax], eax
00007FF60A521DA0  lea          rax, [rcx-1E0h]
00007FF60A521DA7  cmp          rax, 35Fh
00007FF60A521DAD  ja           static void Fabled_Engine::main+12F8h (00007FF60A5225B8h)
00007FF60A521DB3  lea          rbx, [rdx-67h]
00007FF60A521DB7  mov          qword ptr [rsi+rcx*8-F00h], rbx
00007FF60A521DBF  lea          rbp, [rcx-1DFh]
00007FF60A521DC6  cmp          rbp, 35Fh
00007FF60A521DCD  ja           static void Fabled_Engine::main+130Eh (00007FF60A5225CEh)
00007FF60A521DD3  lea          rbp, [rdx-55h]
00007FF60A521DD7  mov          qword ptr [rsi+rcx*8-EF8h], rbp
00007FF60A521DDF  cmp          rax, 35Dh
00007FF60A521DE5  ja           static void Fabled_Engine::main+1324h (00007FF60A5225E4h)
00007FF60A521DEB  lea          rdi, [rdx-66h]
00007FF60A521DEF  mov          qword ptr [rsi+rcx*8-EF0h], rdi
00007FF60A521DF7  mov          qword ptr [rsi+rcx*8-EE8h], rbx
00007FF60A521DFF  cmp          rax, 35Bh
00007FF60A521E05  ja           static void Fabled_Engine::main+133Eh (00007FF60A5225FEh)
00007FF60A521E0B  lea          rax, [rdx-56h]
00007FF60A521E0F  mov          qword ptr [rsi+rcx*8-EE0h], rax
00007FF60A521E17  mov          qword ptr [rsi+rcx*8-ED8h], rbp
00007FF60A521E1F  cmp          rcx, 35Fh
00007FF60A521E26  ja           static void Fabled_Engine::main+1358h (00007FF60A522618h)
00007FF60A521E2C  lea          rbp, [rdx-12h]
00007FF60A521E30  mov          qword ptr [rsi+rcx*8], rbp
00007FF60A521E34  lea          rax, [rcx+1h]
00007FF60A521E38  cmp          rax, 35Fh
00007FF60A521E3E  ja           static void Fabled_Engine::main+136Bh (00007FF60A52262Bh)
00007FF60A521E44  mov          qword ptr [rsi+rcx*8+8h], rdx
00007FF60A521E49  cmp          rcx, 35Dh
00007FF60A521E50  ja           static void Fabled_Engine::main+1381h (00007FF60A522641h)
00007FF60A521E56  lea          rax, [rdx-11h]
00007FF60A521E5A  mov          qword ptr [rsi+rcx*8+10h], rax
00007FF60A521E5F  mov          qword ptr [rsi+rcx*8+18h], rbp
00007FF60A521E64  cmp          rcx, 35Bh
00007FF60A521E6B  ja           static void Fabled_Engine::main+1398h (00007FF60A522658h)
00007FF60A521E71  lea          rax, [rdx-1h]
00007FF60A521E75  mov          qword ptr [rsi+rcx*8+20h], rax
00007FF60A521E7A  mov          qword ptr [rsi+rcx*8+28h], rdx
00007FF60A521E7F  add          rcx, 6h
00007FF60A521E83  add          rdx, 1h
00007FF60A521E87  cmp          rdx, 98h
00007FF60A521E8E  jne          static void Fabled_Engine::main+AE0h (00007FF60A521DA0h)
00007FF60A521E94  mov          edx, 99h
00007FF60A521E99  nop          dword ptr [rax], eax
00007FF60A521EA0  lea          rax, [rcx-1E0h]
00007FF60A521EA7  cmp          rax, 35Fh
00007FF60A521EAD  ja           static void Fabled_Engine::main+12F8h (00007FF60A5225B8h)
00007FF60A521EB3  lea          rbx, [rdx-67h]
00007FF60A521EB7  mov          qword ptr [rsi+rcx*8-F00h], rbx
00007FF60A521EBF  lea          rbp, [rcx-1DFh]
00007FF60A521EC6  cmp          rbp, 35Fh
00007FF60A521ECD  ja           static void Fabled_Engine::main+130Eh (00007FF60A5225CEh)
00007FF60A521ED3  lea          rbp, [rdx-55h]
00007FF60A521ED7  mov          qword ptr [rsi+rcx*8-EF8h], rbp
00007FF60A521EDF  cmp          rax, 35Dh
00007FF60A521EE5  ja           static void Fabled_Engine::main+1324h (00007FF60A5225E4h)
00007FF60A521EEB  lea          rdi, [rdx-66h]
00007FF60A521EEF  mov          qword ptr [rsi+rcx*8-EF0h], rdi
00007FF60A521EF7  mov          qword ptr [rsi+rcx*8-EE8h], rbx
00007FF60A521EFF  cmp          rax, 35Bh
00007FF60A521F05  ja           static void Fabled_Engine::main+133Eh (00007FF60A5225FEh)
00007FF60A521F0B  lea          rax, [rdx-56h]
00007FF60A521F0F  mov          qword ptr [rsi+rcx*8-EE0h], rax
00007FF60A521F17  mov          qword ptr [rsi+rcx*8-ED8h], rbp
00007FF60A521F1F  cmp          rcx, 35Fh
00007FF60A521F26  ja           static void Fabled_Engine::main+1358h (00007FF60A522618h)
00007FF60A521F2C  lea          rbp, [rdx-12h]
00007FF60A521F30  mov          qword ptr [rsi+rcx*8], rbp
00007FF60A521F34  lea          rax, [rcx+1h]
00007FF60A521F38  cmp          rax, 35Fh
00007FF60A521F3E  ja           static void Fabled_Engine::main+136Bh (00007FF60A52262Bh)
00007FF60A521F44  mov          qword ptr [rsi+rcx*8+8h], rdx
00007FF60A521F49  cmp          rcx, 35Dh
00007FF60A521F50  ja           static void Fabled_Engine::main+1381h (00007FF60A522641h)
00007FF60A521F56  lea          rax, [rdx-11h]
00007FF60A521F5A  mov          qword ptr [rsi+rcx*8+10h], rax
00007FF60A521F5F  mov          qword ptr [rsi+rcx*8+18h], rbp
00007FF60A521F64  cmp          rcx, 35Bh
00007FF60A521F6B  ja           static void Fabled_Engine::main+1398h (00007FF60A522658h)
00007FF60A521F71  lea          rax, [rdx-1h]
00007FF60A521F75  mov          qword ptr [rsi+rcx*8+20h], rax
00007FF60A521F7A  mov          qword ptr [rsi+rcx*8+28h], rdx
00007FF60A521F7F  add          rcx, 6h
00007FF60A521F83  add          rdx, 1h
00007FF60A521F87  cmp          rdx, A9h
00007FF60A521F8E  jne          static void Fabled_Engine::main+BE0h (00007FF60A521EA0h)
00007FF60A521F94  mov          ecx, 150h
00007FF60A521F99  mov          edx, 55h
00007FF60A521F9E  nop
00007FF60A521FA0  cmp          rcx, 35Fh
00007FF60A521FA7  ja           static void Fabled_Engine::main+13AFh (00007FF60A52266Fh)
00007FF60A521FAD  lea          rbp, [rdx-12h]
00007FF60A521FB1  mov          qword ptr [rsi+rcx*8], rbp
00007FF60A521FB5  lea          rax, [rcx+1h]
00007FF60A521FB9  cmp          rax, 35Fh
00007FF60A521FBF  ja           static void Fabled_Engine::main+13C2h (00007FF60A522682h)
00007FF60A521FC5  mov          qword ptr [rsi+rcx*8+8h], rdx
00007FF60A521FCA  cmp          rcx, 35Dh
00007FF60A521FD1  ja           static void Fabled_Engine::main+13D8h (00007FF60A522698h)
00007FF60A521FD7  lea          rax, [rdx-11h]
00007FF60A521FDB  mov          qword ptr [rsi+rcx*8+10h], rax
00007FF60A521FE0  mov          qword ptr [rsi+rcx*8+18h], rbp
00007FF60A521FE5  cmp          rcx, 35Bh
00007FF60A521FEC  ja           static void Fabled_Engine::main+13EFh (00007FF60A5226AFh)
00007FF60A521FF2  lea          rax, [rdx-1h]
00007FF60A521FF6  mov          qword ptr [rsi+rcx*8+20h], rax
00007FF60A521FFB  mov          qword ptr [rsi+rcx*8+28h], rdx
00007FF60A522000  add          rcx, 6h
00007FF60A522004  add          rdx, 1h
00007FF60A522008  cmp          rcx, 1B0h
00007FF60A52200F  jne          static void Fabled_Engine::main+CE0h (00007FF60A521FA0h)
00007FF60A522011  mov          edx, 66h
00007FF60A522016  nop          word ptr cs:[rax+rax*1], ax
00007FF60A522020  cmp          rcx, 35Fh
00007FF60A522027  ja           static void Fabled_Engine::main+13AFh (00007FF60A52266Fh)
00007FF60A52202D  lea          rbp, [rdx-12h]
00007FF60A522031  mov          qword ptr [rsi+rcx*8], rbp
00007FF60A522035  lea          rax, [rcx+1h]
00007FF60A522039  cmp          rax, 35Fh
00007FF60A52203F  ja           static void Fabled_Engine::main+13C2h (00007FF60A522682h)
00007FF60A522045  mov          qword ptr [rsi+rcx*8+8h], rdx
00007FF60A52204A  cmp          rcx, 35Dh
00007FF60A522051  ja           static void Fabled_Engine::main+13D8h (00007FF60A522698h)
00007FF60A522057  lea          rax, [rdx-11h]
00007FF60A52205B  mov          qword ptr [rsi+rcx*8+10h], rax
00007FF60A522060  mov          qword ptr [rsi+rcx*8+18h], rbp
00007FF60A522065  cmp          rcx, 35Bh
00007FF60A52206C  ja           static void Fabled_Engine::main+13EFh (00007FF60A5226AFh)
00007FF60A522072  lea          rax, [rdx-1h]
00007FF60A522076  mov          qword ptr [rsi+rcx*8+20h], rax
00007FF60A52207B  mov          qword ptr [rsi+rcx*8+28h], rdx
00007FF60A522080  add          rcx, 6h
00007FF60A522084  add          rdx, 1h
00007FF60A522088  cmp          rdx, 76h
00007FF60A52208C  jne          static void Fabled_Engine::main+D60h (00007FF60A522020h)
00007FF60A52208E  mov          qword ptr [rsp+C0h], 0h
00007FF60A52209A  mov          dword ptr [rsp+C8h], 0h
00007FF60A5220A5  mov          qword ptr [rsp+D0h], 0h
00007FF60A5220B1  mov          dword ptr [rsp+D8h], 0h
00007FF60A5220BC  pxor         xmm0, xmm0
00007FF60A5220C0  movdqa       xmmword ptr [rsp+E0h], xmm0
00007FF60A5220C9  movdqa       xmmword ptr [rsp+F0h], xmm0
00007FF60A5220D2  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A5220D9  test         rcx, rcx
00007FF60A5220DC  jne          static void Fabled_Engine::main+E36h (00007FF60A5220F6h)
00007FF60A5220DE  call         GetProcessHeap (00007FF60A539AA2h)
00007FF60A5220E3  test         rax, rax
00007FF60A5220E6  je           static void Fabled_Engine::main+1489h (00007FF60A522749h)
00007FF60A5220EC  mov          rcx, rax
00007FF60A5220EF  mov          qword ptr [00007FF60A5441C8h], rax
00007FF60A5220F6  mov          r8d, 2E40h
00007FF60A5220FC  xor          edx, edx
00007FF60A5220FE  call         HeapAlloc (00007FF60A539AA8h)
00007FF60A522103  test         rax, rax
00007FF60A522106  je           static void Fabled_Engine::main+1489h (00007FF60A522749h)
00007FF60A52210C  mov          qword ptr [rsp+88h], rax
00007FF60A522114  movaps       xmm0, xmmword ptr [__xmm@000000000000000000000000000000b9 (00007FF60A53D5F0h)]
00007FF60A52211B  movups       xmmword ptr [rsp+90h], xmm0
00007FF60A522123  mov          rcx, qword ptr [rsp+C0h]
00007FF60A52212B  mov          qword ptr [rsp+60h], rcx
00007FF60A522130  mov          ecx, dword ptr [rsp+C8h]
00007FF60A522137  mov          dword ptr [rsp+68h], ecx
00007FF60A52213B  mov          rcx, qword ptr [rsp+D0h]
00007FF60A522143  mov          qword ptr [rsp+78h], rcx
00007FF60A522148  mov          ecx, dword ptr [rsp+D8h]
00007FF60A52214F  mov          dword ptr [rsp+80h], ecx
00007FF60A522156  movaps       xmm0, xmmword ptr [rsp+E0h]
00007FF60A52215E  movaps       xmmword ptr [rsp+40h], xmm0
00007FF60A522163  movaps       xmm0, xmmword ptr [rsp+F0h]
00007FF60A52216B  movaps       xmmword ptr [rsp+30h], xmm0
00007FF60A522170  xor          ecx, ecx
00007FF60A522172  nop          word ptr cs:[rax+rax*1], ax
00007FF60A52217C  nop          dword ptr [rax], eax
00007FF60A522180  mov          edx, dword ptr [rsp+68h]
00007FF60A522184  mov          dword ptr [rax+rcx*1+8h], edx
00007FF60A522188  mov          rdx, qword ptr [rsp+60h]
00007FF60A52218D  mov          qword ptr [rax+rcx*1], rdx
00007FF60A522191  mov          qword ptr [rax+rcx*1+Ch], 0h
00007FF60A52219A  mov          edx, dword ptr [rsp+80h]
00007FF60A5221A1  mov          dword ptr [rax+rcx*1+1Ch], edx
00007FF60A5221A5  mov          rdx, qword ptr [rsp+78h]
00007FF60A5221AA  mov          qword ptr [rax+rcx*1+14h], rdx
00007FF60A5221AF  movaps       xmm0, xmmword ptr [rsp+40h]
00007FF60A5221B4  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF60A5221B9  movaps       xmm0, xmmword ptr [rsp+30h]
00007FF60A5221BE  movaps       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF60A5221C3  add          rcx, 40h
00007FF60A5221C7  cmp          rcx, 2E00h
00007FF60A5221CE  jne          static void Fabled_Engine::main+EC0h (00007FF60A522180h)
00007FF60A5221D0  mov          edx, dword ptr [rsp+68h]
00007FF60A5221D4  mov          dword ptr [rax+rcx*1+8h], edx
00007FF60A5221D8  mov          rdx, qword ptr [rsp+60h]
00007FF60A5221DD  mov          qword ptr [rax+rcx*1], rdx
00007FF60A5221E1  mov          qword ptr [rax+rcx*1+Ch], 0h
00007FF60A5221EA  mov          edx, dword ptr [rsp+80h]
00007FF60A5221F1  mov          dword ptr [rax+rcx*1+1Ch], edx
00007FF60A5221F5  mov          rdx, qword ptr [rsp+78h]
00007FF60A5221FA  mov          qword ptr [rax+rcx*1+14h], rdx
00007FF60A5221FF  movaps       xmm0, xmmword ptr [rsp+40h]
00007FF60A522204  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF60A522209  movaps       xmm0, xmmword ptr [rsp+30h]
00007FF60A52220E  movaps       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF60A522213  mov          qword ptr [rsp+98h], B9h
00007FF60A52221F  mov          rax, qword ptr [rsp+88h]
00007FF60A522227  add          rax, 20h
00007FF60A52222B  xor          ecx, ecx
00007FF60A52222D  xorps        xmm0, xmm0
00007FF60A522230  mov          r8d, dword ptr [r13+rcx*2+8h]
00007FF60A522235  mov          rbp, qword ptr [r13+rcx*2]
00007FF60A52223A  mov          rbx, qword ptr [r15+rcx*1]
00007FF60A52223E  mov          rdi, qword ptr [r12+rcx*2]
00007FF60A522242  mov          edx, dword ptr [r12+rcx*2+8h]
00007FF60A522247  mov          qword ptr [rax+rcx*8-20h], rbp
00007FF60A52224C  mov          dword ptr [rax+rcx*8-18h], r8d
00007FF60A522251  mov          qword ptr [rax+rcx*8-14h], rbx
00007FF60A522256  mov          dword ptr [rax+rcx*8-4h], edx
00007FF60A52225A  mov          qword ptr [rax+rcx*8-Ch], rdi
00007FF60A52225F  movaps       xmmword ptr [rax+rcx*8], xmm0
00007FF60A522263  movaps       xmmword ptr [rax+rcx*8+10h], xmm0
00007FF60A522268  add          rcx, 8h
00007FF60A52226C  cmp          rcx, 5C8h
00007FF60A522273  jne          static void Fabled_Engine::main+F70h (00007FF60A522230h)
00007FF60A522275  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A52227C  test         rcx, rcx
00007FF60A52227F  jne          static void Fabled_Engine::main+FD9h (00007FF60A522299h)
00007FF60A522281  call         GetProcessHeap (00007FF60A539AA2h)
00007FF60A522286  test         rax, rax
00007FF60A522289  je           static void Fabled_Engine::main+1490h (00007FF60A522750h)
00007FF60A52228F  mov          rcx, rax
00007FF60A522292  mov          qword ptr [00007FF60A5441C8h], rax
00007FF60A522299  mov          r8d, 40h
00007FF60A52229F  xor          edx, edx
00007FF60A5222A1  call         HeapAlloc (00007FF60A539AA8h)
00007FF60A5222A6  test         rax, rax
00007FF60A5222A9  je           static void Fabled_Engine::main+1490h (00007FF60A522750h)
00007FF60A5222AF  mov          rbp, rax
00007FF60A5222B2  mov          rax, qword ptr [rsp+98h]
00007FF60A5222BA  mov          qword ptr [rbp+10h], rax
00007FF60A5222BE  movups       xmm0, xmmword ptr [rsp+88h]
00007FF60A5222C6  movups       xmmword ptr [rbp], xmm0
00007FF60A5222CA  mov          qword ptr [rbp+20h], rsi
00007FF60A5222CE  movaps       xmm0, xmmword ptr [__xmm@00000000000003600000000000000360 (00007FF60A53D600h)]
00007FF60A5222D5  movups       xmmword ptr [rbp+28h], xmm0
00007FF60A5222D9  mov          dword ptr [rbp+18h], 0h
00007FF60A5222E0  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A5222E7  xor          edx, edx
00007FF60A5222E9  mov          r8, qword ptr [rsp+B8h]
00007FF60A5222F1  call         HeapFree (00007FF60A539AAEh)
00007FF60A5222F6  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A5222FD  xor          edx, edx
00007FF60A5222FF  mov          r8, qword ptr [rsp+70h]
00007FF60A522304  call         HeapFree (00007FF60A539AAEh)
00007FF60A522309  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A522310  xor          edx, edx
00007FF60A522312  mov          r8, r14
00007FF60A522315  call         HeapFree (00007FF60A539AAEh)
00007FF60A52231A  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A522321  xor          edx, edx
00007FF60A522323  mov          r8, r12
00007FF60A522326  call         HeapFree (00007FF60A539AAEh)
00007FF60A52232B  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A522332  xor          edx, edx
00007FF60A522334  mov          r8, r15
00007FF60A522337  call         HeapFree (00007FF60A539AAEh)
00007FF60A52233C  mov          rcx, qword ptr [00007FF60A5441C8h]
00007FF60A522343  xor          edx, edx
00007FF60A522345  mov          r8, r13
00007FF60A522348  call         HeapFree (00007FF60A539AAEh)
00007FF60A52234D  lea          rax, [00007FF60A540CB0h]
00007FF60A522354  mov          qword ptr [rsp+E0h], rax
00007FF60A52235C  mov          qword ptr [rsp+E8h], 6h
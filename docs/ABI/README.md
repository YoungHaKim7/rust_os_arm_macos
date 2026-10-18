- https://github.com/YoungHaKim7/rust_release/tree/main/rust1_99_ABI/ABI/abi_rust

# API vs ABI 차이점 이해

- https://en.wikipedia.org/wiki/System_call

- [System Call | wiki에 잘 정리](https://en.wikipedia.org/wiki/System_call)

# ABI란 무엇인가
- [(180507)Application Binary Interface (ABI) Docs and Their Meaning](https://kaiwantech.wordpress.com/2018/05/07/application-binary-interface-abi-docs-and-their-meaning/)
  - [(2025년 글) What's up with Rust ABI?](https://www.reddit.com/r/learnrust/comments/1irl30d/whats_up_with_rust_abi/)

# Rust는 C++이 실패한 길로 가면 안된다. 조심하자 러스트
- C++가 실패한걸 배우고 더 좋은 방법을 찾아야한다. 좋은 영상
  - [260225) Why Rust Is Causing Tension in the Linux Kernel. | GaryH Tech](https://youtu.be/-XLuGB0wZ1M?si=kPjXKuDAidG3i6sn)

# Rust만의 길을 걷고 있는 ABI
- https://rust-lang.github.io/rust-project-goals/2025h1/safe-linking.html

- Rust ABI의 현황과 안정화 시 변경될 사항 (Reddit자료 2025년자료)
  - 러스트는 일부러 ABI안정화를 늦추고 있음.(C언어도 문제가 많구만. 어째튼 함부로 안정화 할 필요 없는듯~~) 
  - [(2025년 글) Status of the Rust ABI and What Is Going to Change When It's Stabilized](https://www.reddit.com/r/rust/comments/1hnblpt/status_of_the_rust_abi_and_what_is_going_to/?tl=ko)

<hr />

# ABI 작동원리

# ABI관련 정보의 글
- **[우분투가 Rust를 사용한다는 것은 무엇을 의미할까?](<https://news.hada.io/topic?id=27028&utm_source=discord&utm_medium=bot&utm_campaign=1480>)**
- Ubuntu의 **Rust 도입**은 기술 채택 주기에서 Rust가 **얼리 어답터에서 캐즘을 넘어 주류 단계로 이동**하고 있음을 보여줌  
- Canonical은 Rust를 **새로운 기반 소프트웨어의 기본 언어**로 채택하여 C, C++, 일부 Python 사용을 대체하고 있으며, 메모리 안전 유틸리티 개발에 재정·명성 양면으로 투자 중  
- 초…

# (251121)How the Linux User/Kernel ABI Really Works - Greg Law - C++Now 2025
CppNow
- https://youtu.be/5UkVl0xK-3E?si=rojO_dwDIylEAx2q

# CS 3214: ABI mini lecture
Matthew Hicks
https://youtu.be/8pwbFFBmM-k?si=cSr5SLOSYFxaUm9V

# What Is an ABI, and Why Should You Care? - Shung-Hsi Yu, SUSE
The Linux Foundation
- https://youtu.be/90fwlfon3Hk?si=n9ERHFR7zymhtwo9

# (160116)Syscalls, Kernel vs. User Mode and Linux Kernel Source Code - bin 0x09
LiveOverflow
- https://youtu.be/fLS99zJDHOc?si=Ws5roHzcde28qukl

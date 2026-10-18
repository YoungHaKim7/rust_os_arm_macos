# The Art of Multiprocessor programming
-  The Art of Multiprocessor Programming 2nd Edition
  - by Maurice Herlihy (Author), Nir Shavit (Author), Victor Luchangco (Author), Michael Spear (Author) 
  - https://www.amazon.com/Art-Multiprocessor-Programming-Maurice-Herlihy/dp/0124159508/
    - The Art of Multiprocessor Programming, Second Edition, provides users with an authoritative guide to multicore programming. This updated edition introduces higher level software development skills relative to those needed for efficient single-core programming, and includes comprehensive coverage of the new principles, algorithms, and tools necessary for effective multiprocessor programming. The book is an ideal resource for students and professionals alike who will benefit from its thorough coverage of key multiprocessor programming issues.
      - Features new exercises developed for instructors using the text, with more algorithms, new examples, and other updates throughout the book
      - Presents the fundamentals of programming multiple threads for accessing shared memory
      - Explores mainstream concurrent data structures and the key elements of their design, as well as synchronization techniques, from simple locks to transactional memory systems


<hr />

# This book focuses on how to program multiprocessors that communicate via a shared memory. 
- 이 책은 공유 메모리를 통해 통신하는 멀티프로세서를 프로그래밍하는 방법에 중점을 둡니다.
- Such systems are often called shared-memory multiprocessors or, more recently, multicores. Programming challenges arise at all scales of multiprocessor systems—at a very small scale, processors within a single chip need to coordinate access to a shared memory location, and on a large scale, processors in a supercomputer need to coordinate the routing of data. Multiprocessor programming is challenging because modern computer systems are inherently asynchronous: activities can be halted or delayed without warning by interrupts, preemption, cache misses, failures, and other events. These delays are inherently unpredictable, and can vary enormously in scale: a cache miss might delay a processor for fewer than ten instructions, a page fault for a few million instructions, and operating system preemption for hundreds of millions of instructions.
  -  이러한 시스템은 종종 공유 메모리 멀티프로세서 또는 최근에는 멀티코어라고 불립니다. 프로그래밍 문제는 모든 규모의 멀티프로세서 시스템에서 발생합니다. 아주 작은 규모에서는 단일 칩 내의 프로세서가 공유 메모리 위치에 대한 접근을 조정해야 하고, 큰 규모에서는 슈퍼컴퓨터의 프로세서가 데이터 라우팅을 조정해야 합니다. 멀티프로세서 프로그래밍은 현대 컴퓨터 시스템이 본질적으로 비동기적이기 때문에 도전적입니다. 활동은 중단, 선점, 캐시 누락, 실패 및 기타 이벤트로 인해 경고 없이 중단되거나 지연될 수 있습니다. 이러한 지연은 본질적으로 예측할 수 없으며, 크기는 매우 다양할 수 있습니다. 캐시 누락은 프로세서를 10개 미만의 명령어로 지연시킬 수 있으며, 페이지 오류는 몇 백만 개의 명령어로 지연시킬 수 있고, 운영 체제는 수억 개의 명령어로 인해 지연될 수 있습니다.
- We approach multiprocessor programming from two complementary directions: principles and practice. In the principles part of this book, we focus on computability: figuring out what can be computed in an asynchronous concurrent environment.
  - 우리는 멀티프로세서 프로그래밍을 두 가지 상호 보완적인 방향, 즉 원리와 실천에서 접근합니다. 이 책의 원리 부분에서는 비동기 동시 환경에서 계산할 수 있는 것을 알아내는 계산 가능성에 중점을 둡니다.

# **[퓨텍스 없이는 의미 없다](<https://news.hada.io/topic?id=22642&utm_source=discord&utm_medium=bot&utm_campaign=1480>)**
- **The Art of Multiprocessor Programming** 교재가 **퓨텍스(futex)** 개념을 다루지 않아 아쉽다는 문제 제기  
- **Futex**는 현대 병렬 프로그래밍에서 **효율적인 동기화**의 핵심 구성요소로, 기존 System V 기반 락보다 뛰어난 성능을 보임  
- 퓨텍스는 **락 획득과 대기/깨우기 기능을 분리**하여, 불필요한 …

- futex예시 코드(c언어)
  - https://codeberg.org/crashoverride/mutex

- 러스트(Rust)에서 구현한 futex
  - https://doc.rust-lang.org/nightly/src/std/sys/sync/thread_parking/futex.rs.html
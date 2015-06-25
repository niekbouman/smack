// Tests that using mutex fails after being destroyed

// @expect verified

#include <pthread.h>

pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;

int x = 1;

void *t1(void *arg) {
  pthread_mutex_lock(&lock);
  x++;
  pthread_mutex_unlock(&lock);
}

int main() {

  pthread_t tid1;
  assert(lock.lock == UNLOCKED);
  assert(lock.init == INITIALIZED);

  pthread_create(&tid1, 0, t1, 0);
  pthread_mutex_lock(&lock);
  x++;
  pthread_mutex_unlock(&lock);
  pthread_join(tid1, 0);
  assert(x == 3);
}

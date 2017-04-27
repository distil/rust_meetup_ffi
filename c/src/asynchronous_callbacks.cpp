#include <cstdint>
#include <thread>

typedef void (*callback_t)(void *, int32_t);

extern "C" void c_perform_task(int32_t value, callback_t callback, void *userdata) {
    std::thread([=]() {
        std::this_thread::sleep_for(std::chrono::seconds(3));
        callback(userdata, value * value);
    }).join();
}

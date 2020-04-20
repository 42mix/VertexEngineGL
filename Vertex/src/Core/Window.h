#pragma once

#include "Core.hpp"

#include "Event/Event.h"

namespace Vertex {

    class Window
    {
    public:
        virtual void OnUpdate() = 0;

        virtual unsigned int GetWidth() const = 0;
        virtual unsigned int GetHeight() const = 0;

        template<typename T>
        virtual void SetEventCallback(T&& func) = 0;

        virtual void SetVSync(bool conf) = 0;
        virtual bool IsVSync() const = 0;

        virtual void* GetNativeWindow() const = 0;
    };

}
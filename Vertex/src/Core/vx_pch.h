/**
 * Precompiled header: this will be included automatically for all files.
 */

#pragma once

#include <map>
#include <memory>
#include <vector>
#include <cassert>
#include <stdexcept>
#include <functional>

#include "Core.h"
#include "Logger.h"

#if VX_RENDER_API == VX_RENDER_API_OPENGL
#include <glad/glad.h>
#elif VX_RENDER_API == VX_RENDER_API_VULKAN
#define GLFW_INCLUDE_VULKAN
#endif
#include <GLFW/glfw3.h>

#include <glm/glm.hpp>
#include <glm/ext.hpp>
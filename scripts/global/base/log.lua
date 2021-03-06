--log.lua
--Created by wugd
--日志类相关的信息

module("LOG", package.seeall)

LOG_ERROR = 1
LOG_WARN = 2
LOG_INFO = 3
LOG_DEBUG = 4
LOG_TRACE = 5

lua_print = _G.tel_print

local function format(value, ...)
    local a = {...}
    local i = 0
    if(type(value) == "string") then
        value = string.gsub(value,"%%([o,s,d])",function(c)
                                                    i = i+1
                                                    if c == "s" then
                                                        return a[i]
                                                    else
                                                        return (watch(a[i]))
                                                    end
                                                end)
    end

    for idx = i + 1, #a do
        value = value .. string.format(" args : %d, value : %s", idx, watch(a[i]))
    end
    return value
end

local function get_log_level()
    if LOG_LEVEL then
        return tonumber(LOG_LEVEL) or LOG_WARN
    end
    return LOG_WARN
end

function err(value, ...)
    if get_log_level() < LOG_ERROR then
        return
    end

    value = format(value, ...)
    lua_print(value)
    write_log(value)
end

function warn(value, ...)
    if get_log_level() < LOG_WARN then
        return
    end

    value = format(value, ...)
    lua_print(value)
end

function info(value, ...)
    if get_log_level() < LOG_INFO then
        return
    end

    value = format(value, ...)
    lua_print(value)
end

function debug(value, ...)
    if get_log_level() < LOG_DEBUG then
        return
    end

    value = format(value, ...)
    lua_print(value)
end

function trace(value, ...)
    -- if get_log_level() < LOG_TRACE then
    --     return
    -- end

    value = format(value, ...)
    lua_print(value)
    write_log(value)
end

if print then
    print = trace
end

if _G.trace then
    _G.trace = trace
end
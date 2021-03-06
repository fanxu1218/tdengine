--redis_queued.lua
--redis消息队列处理
module("REDIS_QUEUED", package.seeall)

local first_msg = "pmessage"
function deal_with_reply(reply)
    if not is_table(reply) then
        return
    end
    trace("__ REDIS_QUEUED:deal_with_reply() __ is %o \n", reply)
end

function deal_with_respone_list(respone_list)
    for _,reply in ipairs(respone_list) do
        deal_with_reply(reply)
    end
end

local function time_update()
    local respone_list = REDIS_D.subs_get_reply()
    if respone_list ~= nil and #respone_list > 0 then
        deal_with_respone_list(respone_list)
    end
end

function create()
    set_timer(10000, time_update, nil, true)
end

create()
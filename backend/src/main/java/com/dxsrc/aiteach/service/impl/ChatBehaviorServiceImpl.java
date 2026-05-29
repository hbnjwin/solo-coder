package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.ChatLog;
import com.dxsrc.aiteach.mapper.ChatLogMapper;
import com.dxsrc.aiteach.service.ChatBehaviorService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class ChatBehaviorServiceImpl extends ServiceImpl<ChatLogMapper, ChatLog> implements ChatBehaviorService {

    @Override
    public List<ChatLog> list() {
        return super.list();
    }

    @Override
    public ChatLog getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(ChatLog entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(ChatLog entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}

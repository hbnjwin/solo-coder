package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.ChatSession;
import com.dxsrc.aiteach.mapper.ChatSessionMapper;
import com.dxsrc.aiteach.service.ChatMemoryService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class ChatMemoryServiceImpl extends ServiceImpl<ChatSessionMapper, ChatSession> implements ChatMemoryService {

    @Override
    public List<ChatSession> list() {
        return super.list();
    }

    @Override
    public ChatSession getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(ChatSession entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(ChatSession entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}

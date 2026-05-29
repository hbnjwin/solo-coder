package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.ChatSession;
import java.util.List;

public interface ChatMemoryService {

    List<ChatSession> list();

    ChatSession getById(Long id);

    boolean save(ChatSession entity);

    boolean update(ChatSession entity);

    boolean removeById(Long id);
}

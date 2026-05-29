package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.ChatLog;
import java.util.List;

public interface ChatBehaviorService {

    List<ChatLog> list();

    ChatLog getById(Long id);

    boolean save(ChatLog entity);

    boolean update(ChatLog entity);

    boolean removeById(Long id);
}

package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.ChatLog;
import com.dxsrc.aiteach.service.ChatBehaviorService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/chat-behavior")
@RequiredArgsConstructor
public class ChatBehaviorController {

    private final ChatBehaviorService chatBehaviorService;

    @GetMapping("/list")
    public List<ChatLog> list() {
        return chatBehaviorService.list();
    }

    @GetMapping("/{id}")
    public ChatLog getById(@PathVariable Long id) {
        return chatBehaviorService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody ChatLog entity) {
        return chatBehaviorService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody ChatLog entity) {
        return chatBehaviorService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return chatBehaviorService.removeById(id);
    }
}

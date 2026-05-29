package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.ChatSession;
import com.dxsrc.aiteach.service.ChatMemoryService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/chat-memory")
@RequiredArgsConstructor
public class ChatMemoryController {

    private final ChatMemoryService chatMemoryService;

    @GetMapping("/list")
    public List<ChatSession> list() {
        return chatMemoryService.list();
    }

    @GetMapping("/{id}")
    public ChatSession getById(@PathVariable Long id) {
        return chatMemoryService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody ChatSession entity) {
        return chatMemoryService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody ChatSession entity) {
        return chatMemoryService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return chatMemoryService.removeById(id);
    }
}

package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Workflow;
import com.dxsrc.aiteach.service.WorkflowDebugService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/workflow-debug")
@RequiredArgsConstructor
public class WorkflowDebugController {

    private final WorkflowDebugService workflowDebugService;

    @GetMapping("/list")
    public List<Workflow> list() {
        return workflowDebugService.list();
    }

    @GetMapping("/{id}")
    public Workflow getById(@PathVariable Long id) {
        return workflowDebugService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Workflow entity) {
        return workflowDebugService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Workflow entity) {
        return workflowDebugService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return workflowDebugService.removeById(id);
    }
}

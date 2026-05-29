package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Experiment;
import com.dxsrc.aiteach.service.MlExperimentService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/ml-experiment")
@RequiredArgsConstructor
public class MlExperimentController {

    private final MlExperimentService mlExperimentService;

    @GetMapping("/list")
    public List<Experiment> list() {
        return mlExperimentService.list();
    }

    @GetMapping("/{id}")
    public Experiment getById(@PathVariable Long id) {
        return mlExperimentService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Experiment entity) {
        return mlExperimentService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Experiment entity) {
        return mlExperimentService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return mlExperimentService.removeById(id);
    }
}

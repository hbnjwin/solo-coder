package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Exam;
import com.dxsrc.aiteach.mapper.ExamMapper;
import com.dxsrc.aiteach.service.CorrectionAnalysisService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class CorrectionAnalysisServiceImpl extends ServiceImpl<ExamMapper, Exam> implements CorrectionAnalysisService {

    @Override
    public List<Exam> list() {
        return super.list();
    }

    @Override
    public Exam getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Exam entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Exam entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}

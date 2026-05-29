package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.TranslationProject;
import com.dxsrc.aiteach.mapper.TranslationProjectMapper;
import com.dxsrc.aiteach.service.TranslationService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class TranslationServiceImpl extends ServiceImpl<TranslationProjectMapper, TranslationProject> implements TranslationService {

    @Override
    public List<TranslationProject> list() {
        return super.list();
    }

    @Override
    public TranslationProject getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(TranslationProject entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(TranslationProject entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}

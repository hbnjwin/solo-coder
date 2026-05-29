package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.StyleTemplate;
import com.dxsrc.aiteach.mapper.StyleTemplateMapper;
import com.dxsrc.aiteach.service.StyleTransferService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class StyleTransferServiceImpl extends ServiceImpl<StyleTemplateMapper, StyleTemplate> implements StyleTransferService {

    @Override
    public List<StyleTemplate> list() {
        return super.list();
    }

    @Override
    public StyleTemplate getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(StyleTemplate entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(StyleTemplate entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}

package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Mindmap;
import com.dxsrc.aiteach.mapper.MindmapMapper;
import com.dxsrc.aiteach.service.MindmapCollabService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class MindmapCollabServiceImpl extends ServiceImpl<MindmapMapper, Mindmap> implements MindmapCollabService {

    @Override
    public List<Mindmap> list() {
        return super.list();
    }

    @Override
    public Mindmap getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Mindmap entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Mindmap entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}

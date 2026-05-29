package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.MusicProject;
import com.dxsrc.aiteach.mapper.MusicProjectMapper;
import com.dxsrc.aiteach.service.MusicCollabService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class MusicCollabServiceImpl extends ServiceImpl<MusicProjectMapper, MusicProject> implements MusicCollabService {

    @Override
    public List<MusicProject> list() {
        return super.list();
    }

    @Override
    public MusicProject getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(MusicProject entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(MusicProject entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}

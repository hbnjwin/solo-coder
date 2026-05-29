package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Video;
import com.dxsrc.aiteach.mapper.VideoMapper;
import com.dxsrc.aiteach.service.VideoSubtitleService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class VideoSubtitleServiceImpl extends ServiceImpl<VideoMapper, Video> implements VideoSubtitleService {

    @Override
    public List<Video> list() {
        return super.list();
    }

    @Override
    public Video getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Video entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Video entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}

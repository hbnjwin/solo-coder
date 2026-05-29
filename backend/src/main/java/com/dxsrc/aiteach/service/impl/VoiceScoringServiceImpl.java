package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.DialogueSession;
import com.dxsrc.aiteach.mapper.DialogueSessionMapper;
import com.dxsrc.aiteach.service.VoiceScoringService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class VoiceScoringServiceImpl extends ServiceImpl<DialogueSessionMapper, DialogueSession> implements VoiceScoringService {

    @Override
    public List<DialogueSession> list() {
        return super.list();
    }

    @Override
    public DialogueSession getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(DialogueSession entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(DialogueSession entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}

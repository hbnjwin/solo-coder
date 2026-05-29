package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Document;
import com.dxsrc.aiteach.mapper.DocumentMapper;
import com.dxsrc.aiteach.service.WriteCollabService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class WriteCollabServiceImpl extends ServiceImpl<DocumentMapper, Document> implements WriteCollabService {

    @Override
    public List<Document> list() {
        return super.list();
    }

    @Override
    public Document getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Document entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Document entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}

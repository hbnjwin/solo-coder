package com.linkyoyo.tobacco.service.impl;

import com.linkyoyo.tobacco.entity.StandardDocToc;
import com.linkyoyo.tobacco.repository.StandardDocTocRepository;
import com.linkyoyo.tobacco.service.StandardDocTocService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.ZonedDateTime;
import java.util.List;

/**
 * 标准文档目次服务实现
 */
@Service
public class StandardDocTocServiceImpl implements StandardDocTocService {

    @Autowired
    private StandardDocTocRepository standardDocTocRepository;

    @Override
    public List<StandardDocToc> getTableOfContentsByStandardDocId(Integer standardDocId) {
        return standardDocTocRepository.findByStandardDocIdOrderByDisplayOrder(standardDocId);
    }

    @Override
    @Transactional
    public Integer batchSaveTableOfContents(Integer standardDocId, List<StandardDocToc> tocEntries) {
        // 先删除已有的目次
        standardDocTocRepository.deleteByStandardDocId(standardDocId);

        // 设置创建和更新时间
        ZonedDateTime now = ZonedDateTime.now();
        int displayOrder = 1;

        // 批量保存目次条目
        for (StandardDocToc entry : tocEntries) {
            entry.setStandardDocId(standardDocId);
            entry.setCreatedAt(now);
            entry.setUpdatedAt(now);
            entry.setDisplayOrder(displayOrder++);
            standardDocTocRepository.save(entry);
        }

        return tocEntries.size();
    }

    @Override
    @Transactional
    public StandardDocToc addTableOfContentsEntry(StandardDocToc standardDocToc) {
        // 设置创建和更新时间
        ZonedDateTime now = ZonedDateTime.now();
        standardDocToc.setCreatedAt(now);
        standardDocToc.setUpdatedAt(now);

        // 如果没有设置显示顺序，则设置为当前最大显示顺序+1
        if (standardDocToc.getDisplayOrder() == null) {
            List<StandardDocToc> existingEntries = standardDocTocRepository.findByStandardDocIdOrderByDisplayOrder(standardDocToc.getStandardDocId());
            int maxOrder = existingEntries.isEmpty() ? 0 : existingEntries.get(existingEntries.size() - 1).getDisplayOrder();
            standardDocToc.setDisplayOrder(maxOrder + 1);
        }

        return standardDocTocRepository.save(standardDocToc);
    }

    @Override
    @Transactional
    public StandardDocToc updateTableOfContentsEntry(StandardDocToc standardDocToc) {
        // 设置更新时间
        standardDocToc.setUpdatedAt(ZonedDateTime.now());

        return standardDocTocRepository.save(standardDocToc);
    }

    @Override
    @Transactional
    public void deleteTableOfContentsEntry(Integer id) {
        standardDocTocRepository.deleteById(id);
    }

    @Override
    @Transactional
    public void deleteAllTableOfContentsByStandardDocId(Integer standardDocId) {
        standardDocTocRepository.deleteByStandardDocId(standardDocId);
    }
}
